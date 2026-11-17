//! `Engine`, `EngineHandle` and the control thread (04 §5; 06 §3, §5.1).
//!
//! Each call of the `EngineHandle` that needs the control thread sends it a message through a
//! channel; the control thread handles them in order of arrival and turns them into commands
//! for the output thread (06 §5.2). It goes round its loop every 5 ms even without messages.

use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::audio_pool::AudioPool;
use crate::constants::KIT_ROWS;
use crate::error::EngineError;
use crate::io::devices::{ActiveDevices, DeviceList};
use crate::io::output::{open_default_output, OutputStream};
use crate::kits::{load_kits, Kits};
use crate::project::{EngineProject, Preparer};
use crate::rt::commands::{command_queue, CommandProducer, PlayMode, RtCommand};
use crate::rt::processor::Processor;
use crate::snapshot::EngineSnapshot;
use crate::telemetry::{EngineEvent, SharedState, Telemetry};

/// How often the control thread goes round its loop without messages (06 §3).
const CONTROL_TICK: Duration = Duration::from_millis(5);
/// How long the control thread waits before trying again when the command queue is full
/// (06 §5.2).
const FULL_QUEUE_RETRY: Duration = Duration::from_millis(1);
/// Detail of the methods whose task has not been done yet (09 §1.3, rule 7).
const NOT_IMPLEMENTED: &str = "not implemented";

/// Configuration of the engine (04 §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    /// Folder with the three kits (07 §6.1).
    pub kits_dir: PathBuf,
}

/// What to play (04 §6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "mode",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum PlayRequest {
    /// Song mode, from a frame of the song.
    Song { start_frame: u64 },
    /// Pattern mode: a pattern of a rhythm track, in a loop.
    Pattern {
        track_id: String,
        pattern_id: String,
    },
}

/// A recording to start (04 §6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordRequest {
    pub track_id: String,
    pub start_frame: u64,
    pub project_dir: PathBuf,
}

/// A recording that has started (04 §5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordStarted {
    /// Relative path of the take: `"audio/take-0003.wav"`.
    pub file: String,
}

/// The audio engine. It owns its threads: it is created once, when the program starts, and
/// stops them when it is dropped.
pub struct Engine {
    handle: EngineHandle,
    control: Option<JoinHandle<()>>,
}

/// Opens the output stream for a new `Processor`, or gives `None` if there is no output.
type OpenOutput = fn(Processor, Arc<SharedState>) -> Option<OutputStream>;

/// Slot of each track of the last snapshot, in its order (06 §18.1). Only the control thread
/// and `EngineHandle::telemetry()` use it; never an audio thread.
type TrackSlots = Arc<Mutex<Vec<(String, usize)>>>;

impl Engine {
    /// Loads the three kits (06 §8.4), starts the control thread and opens Windows' default
    /// output device. Without an output device it starts all the same, with
    /// `output_available` set to `false` (06 §13.1). Fails if a kit cannot be loaded.
    pub fn start(config: EngineConfig) -> Result<Engine, EngineError> {
        Self::start_with(config, open_default_output)
    }

    /// Starts the engine opening the output with `open_output`.
    pub(crate) fn start_with(
        config: EngineConfig,
        open_output: OpenOutput,
    ) -> Result<Engine, EngineError> {
        let kits = Arc::new(load_kits(&config.kits_dir)?);
        let shared = Arc::new(SharedState::new());
        let events = Arc::new(Mutex::new(Vec::new()));
        let track_slots: TrackSlots = Arc::new(Mutex::new(Vec::new()));
        let (messages, receiver) = mpsc::channel();
        let (ready, started) = mpsc::channel();

        let control = {
            let shared = Arc::clone(&shared);
            let events = Arc::clone(&events);
            let track_slots = Arc::clone(&track_slots);
            thread::Builder::new()
                .name("carabo-control".to_string())
                .spawn(move || {
                    let mut control =
                        Control::start(kits, shared, events, track_slots, open_output);
                    // The engine waits for this message; if it has gone, there is no one to tell.
                    let _ = ready.send(());
                    control.run(&receiver);
                })
                .map_err(|error| {
                    EngineError::EngineInternal(format!("cannot start the control thread: {error}"))
                })?
        };
        started.recv().map_err(|_| {
            EngineError::EngineInternal("the control thread stopped while starting".to_string())
        })?;

        Ok(Engine {
            handle: EngineHandle {
                messages,
                shared,
                events,
                track_slots,
            },
            control: Some(control),
        })
    }

    /// A handle to talk to the engine from any thread that is not an audio thread.
    pub fn handle(&self) -> EngineHandle {
        self.handle.clone()
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        // The output stream closes when the control thread ends.
        let _ = self.handle.messages.send(Message::Shutdown);
        if let Some(control) = self.control.take() {
            let _ = control.join();
        }
    }
}

/// Talks to the engine. It can be cloned and used from any thread that is not an audio thread
/// (06 §5.1).
#[derive(Clone)]
pub struct EngineHandle {
    messages: Sender<Message>,
    shared: Arc<SharedState>,
    events: Arc<Mutex<Vec<EngineEvent>>>,
    track_slots: TrackSlots,
}

impl EngineHandle {
    /// Validates `snapshot` on the calling thread (06 §5.5) and hands it to the control thread,
    /// which prepares it and delivers it to the output thread (06 §5.3). An invalid snapshot
    /// gives `INVALID_SNAPSHOT` and the engine goes on with the previous one.
    pub fn set_project(&self, snapshot: EngineSnapshot) -> Result<(), EngineError> {
        snapshot.validate()?;
        self.request(|reply| Message::SetProject(snapshot, reply))
    }

    /// Starts playing in the requested mode. `NO_OUTPUT_DEVICE` without an output stream. In
    /// pattern mode, if the track or the pattern is not in the last snapshot, it does nothing
    /// (06 §5.1).
    pub fn play(&self, request: PlayRequest) -> Result<(), EngineError> {
        match request {
            // TODO(T-043): song mode.
            PlayRequest::Song { .. } => not_implemented(),
            PlayRequest::Pattern {
                track_id,
                pattern_id,
            } => self.request(|reply| Message::PlayPattern {
                track_id,
                pattern_id,
                reply,
            }),
        }
    }

    /// Stops the playback, the count-in or the recording.
    pub fn stop(&self) {
        // If the engine has stopped, there is nothing to stop.
        let _ = self.messages.send(Message::Stop);
    }

    /// Moves the playback in song mode to `frame`.
    // TODO(T-046)
    pub fn seek(&self, _frame: u64) {}

    /// Sets the state and the range of the loop.
    // TODO(T-046)
    pub fn set_loop(&self, _enabled: bool, _start_frame: u64, _end_frame: u64) {}

    /// Turns the metronome on or off.
    // TODO(T-047)
    pub fn set_metronome(&self, _enabled: bool) {}

    /// Plays the sound of a row of a rhythm track once (06 §8.3). Without an output stream,
    /// or if the track does not exist or is not a rhythm track, it does nothing.
    pub fn audition(&self, track_id: &str, row: usize) {
        // If the engine has stopped, there is nothing to play.
        let _ = self.messages.send(Message::Audition {
            track_id: track_id.to_string(),
            row,
        });
    }

    /// Starts the count-in and then records and plays from `request.start_frame` (06 §14.4).
    // TODO(T-083)
    pub fn record_start(&self, _request: RecordRequest) -> Result<RecordStarted, EngineError> {
        not_implemented()
    }

    /// Opens the input stream if it is not open (06 §13.3).
    // TODO(T-081)
    pub fn input_open(&self) -> Result<(), EngineError> {
        not_implemented()
    }

    /// Closes the input stream. With a capture in progress it does nothing.
    // TODO(T-081)
    pub fn input_close(&self) {}

    /// The output and input devices of the system (06 §13.1).
    // TODO(T-080)
    pub fn devices(&self) -> DeviceList {
        DeviceList::default()
    }

    /// Changes the devices (`None`: Windows' default) and returns those in use (06 §13.1).
    // TODO(T-080)
    pub fn select_devices(
        &self,
        _output: Option<String>,
        _input: Option<String>,
    ) -> Result<ActiveDevices, EngineError> {
        not_implemented()
    }

    /// Measures the round-trip delay, in frames (06 §15).
    // TODO(T-085)
    pub fn calibrate(&self) -> Result<u32, EngineError> {
        not_implemented()
    }

    /// Plays the test tone (06 §12). `NO_OUTPUT_DEVICE` without an output stream.
    pub fn test_tone(&self) -> Result<(), EngineError> {
        self.request(Message::TestTone)
    }

    /// Reads the atomic values of the telemetry at once, without blocking and without going
    /// through the control thread (06 §18.1).
    pub fn telemetry(&self) -> Telemetry {
        let track_slots = self
            .track_slots
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        self.shared.telemetry(&track_slots)
    }

    /// Returns and empties the pending events (06 §18.2).
    pub fn poll_events(&self) -> Vec<EngineEvent> {
        let mut events = self.events.lock().unwrap_or_else(PoisonError::into_inner);
        std::mem::take(&mut *events)
    }

    /// The audio already loaded into memory, to share it with the export (06 §9.1).
    // TODO(T-044)
    pub fn audio_pool(&self) -> AudioPool {
        AudioPool::default()
    }

    /// The engine clock (06 §6.2), for the tests.
    pub(crate) fn engine_clock(&self) -> u64 {
        self.shared.engine_clock()
    }

    /// Sends a message that carries where to reply and waits for the reply.
    fn request<T>(
        &self,
        message: impl FnOnce(Sender<Result<T, EngineError>>) -> Message,
    ) -> Result<T, EngineError> {
        let (reply, response) = mpsc::channel();
        let stopped = || EngineError::EngineInternal("the engine has stopped".to_string());
        self.messages.send(message(reply)).map_err(|_| stopped())?;
        response.recv().map_err(|_| stopped())?
    }
}

fn not_implemented<T>() -> Result<T, EngineError> {
    Err(EngineError::EngineInternal(NOT_IMPLEMENTED.to_string()))
}

fn no_output_device() -> EngineError {
    EngineError::NoOutputDevice("there is no output stream".to_string())
}

/// Where the control thread sends the result of a message.
type Reply = Sender<Result<(), EngineError>>;

/// A message from an `EngineHandle` (or from the `Engine`) to the control thread.
enum Message {
    /// A snapshot that is already valid.
    SetProject(EngineSnapshot, Reply),
    PlayPattern {
        track_id: String,
        pattern_id: String,
        reply: Reply,
    },
    Stop,
    Audition {
        track_id: String,
        row: usize,
    },
    TestTone(Reply),
    Shutdown,
}

/// The output stream and the end of the command queue that feeds its `Processor`.
struct Output {
    commands: CommandProducer,
    // Kept open while the control thread has it (06 §3).
    _stream: OutputStream,
}

/// The state of the control thread.
struct Control {
    shared: Arc<SharedState>,
    events: Arc<Mutex<Vec<EngineEvent>>>,
    track_slots: TrackSlots,
    output: Option<Output>,
    /// The slots of the tracks and the last snapshot (06 §5.3, §5.4).
    preparer: Preparer,
    /// The prepared snapshots sent to the output thread. The control thread keeps a reference
    /// to each one, so that the output thread never frees one (RT-02), and drops those that
    /// only it still holds.
    // TODO(T-033): the trash queue (06 §5.6) takes the place of this list.
    sent_projects: Vec<Arc<EngineProject>>,
}

impl Control {
    /// Creates a `Processor` with its command queue and opens the output stream for it
    /// (06 §13.1, §13.2).
    fn start(
        kits: Arc<Kits>,
        shared: Arc<SharedState>,
        events: Arc<Mutex<Vec<EngineEvent>>>,
        track_slots: TrackSlots,
        open_output: OpenOutput,
    ) -> Self {
        let (commands, consumer) = command_queue();
        let processor = Processor::new(consumer, Arc::clone(&shared), kits);
        let opened = {
            let shared = Arc::clone(&shared);
            panic::catch_unwind(AssertUnwindSafe(|| open_output(processor, shared)))
        };
        let output = opened.ok().flatten().map(|stream| Output {
            commands,
            _stream: stream,
        });
        shared.set_output_available(output.is_some());
        Self {
            shared,
            events,
            track_slots,
            output,
            preparer: Preparer::new(),
            sent_projects: Vec::new(),
        }
    }

    /// The loop of the control thread. A panic is caught at its border: the transport stops,
    /// `InternalError` is emitted and the loop goes on (06 §20).
    fn run(&mut self, messages: &Receiver<Message>) {
        loop {
            let message = match messages.recv_timeout(CONTROL_TICK) {
                Ok(Message::Shutdown) | Err(RecvTimeoutError::Disconnected) => return,
                Ok(message) => Some(message),
                Err(RecvTimeoutError::Timeout) => None,
            };
            let turn = panic::catch_unwind(AssertUnwindSafe(|| {
                if let Some(message) = message {
                    self.handle(message);
                }
                self.watch_audio_thread();
            }));
            if turn.is_err() {
                self.internal_error("panic in the control thread");
            }
            self.sent_projects
                .retain(|project| Arc::strong_count(project) > 1);
        }
    }

    fn handle(&mut self, message: Message) {
        // The caller of a message with a reply may have gone; then no one waits for it.
        match message {
            Message::SetProject(snapshot, reply) => {
                let project = Arc::new(self.preparer.prepare(snapshot));
                *self
                    .track_slots
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner) = self.preparer.track_slots();
                if self.send(RtCommand::SetProject(Arc::clone(&project))) {
                    self.sent_projects.push(project);
                }
                let _ = reply.send(Ok(()));
            }
            Message::PlayPattern {
                track_id,
                pattern_id,
                reply,
            } => {
                let result = if self.output.is_some() {
                    if let Some((slot, pattern_index)) =
                        self.preparer.pattern(&track_id, &pattern_id)
                    {
                        self.send(RtCommand::Play(PlayMode::Pattern {
                            slot,
                            pattern_index,
                        }));
                    }
                    Ok(())
                } else {
                    Err(no_output_device())
                };
                let _ = reply.send(result);
            }
            Message::Stop => {
                self.send(RtCommand::Stop);
            }
            Message::Audition { track_id, row } => {
                let slot = self.preparer.rhythm_slot(&track_id);
                if let Some(slot) = slot.filter(|_| row < KIT_ROWS) {
                    self.send(RtCommand::Audition { slot, row });
                }
            }
            Message::TestTone(reply) => {
                let result = if self.send(RtCommand::TestTone) {
                    Ok(())
                } else {
                    Err(no_output_device())
                };
                let _ = reply.send(result);
            }
            Message::Shutdown => {}
        }
    }

    /// Sends a command to the output thread. If the queue is full, it waits 1 ms and tries
    /// again: a command is never dropped (06 §5.2). `false` if there is no output stream.
    fn send(&mut self, mut command: RtCommand) -> bool {
        let Some(output) = self.output.as_mut() else {
            return false;
        };
        loop {
            match output.commands.push(command) {
                Ok(()) => return true,
                Err(rtrb::PushError::Full(rejected)) => {
                    command = rejected;
                    thread::sleep(FULL_QUEUE_RETRY);
                }
            }
        }
    }

    /// An audio thread has met an impossible state: the transport stops and `InternalError` is
    /// emitted once per incident (06 §20).
    fn watch_audio_thread(&mut self) {
        if self.shared.take_internal_error() {
            self.internal_error("impossible state in an audio thread");
        }
    }

    fn internal_error(&mut self, detail: &str) {
        self.send(RtCommand::Stop);
        self.events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(EngineEvent::InternalError {
                detail: detail.to_string(),
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn play_requests_deserialize_from_the_json_of_04_6_1() {
        let song: PlayRequest =
            serde_json::from_value(json!({ "mode": "song", "startFrame": 96000 }))
                .expect("a song request deserializes");
        assert_eq!(
            song,
            PlayRequest::Song {
                start_frame: 96_000
            }
        );

        let pattern: PlayRequest = serde_json::from_value(
            json!({ "mode": "pattern", "trackId": "t1", "patternId": "p2" }),
        )
        .expect("a pattern request deserializes");
        assert_eq!(
            pattern,
            PlayRequest::Pattern {
                track_id: "t1".to_string(),
                pattern_id: "p2".to_string()
            }
        );
    }

    #[test]
    fn record_requests_and_results_use_the_keys_of_04_6_1() {
        let request: RecordRequest = serde_json::from_value(
            json!({ "trackId": "t3", "startFrame": 480, "projectDir": "C:/Songs/Demo" }),
        )
        .expect("a record request deserializes");
        assert_eq!(
            request,
            RecordRequest {
                track_id: "t3".to_string(),
                start_frame: 480,
                project_dir: PathBuf::from("C:/Songs/Demo"),
            }
        );
        let started = RecordStarted {
            file: "audio/take-0003.wav".to_string(),
        };
        assert_eq!(
            serde_json::to_value(&started).expect("a record result serializes"),
            json!({ "file": "audio/take-0003.wav" })
        );
    }

    #[test]
    fn the_engine_and_its_handle_can_be_shared_between_threads() {
        fn shareable<T: Send + Sync + 'static>() {}
        shareable::<Engine>();
        shareable::<EngineHandle>();
    }

    #[test]
    fn an_impossible_state_in_the_audio_thread_gives_one_internal_error() {
        let engine = Engine::start_with(
            EngineConfig {
                kits_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/kits"),
            },
            |_, _| None,
        )
        .expect("the engine starts");
        let handle = engine.handle();
        handle.shared.raise_internal_error();

        let mut events = Vec::new();
        for _ in 0..200 {
            events.extend(handle.poll_events());
            if !events.is_empty() {
                break;
            }
            thread::sleep(CONTROL_TICK);
        }
        thread::sleep(4 * CONTROL_TICK);
        events.extend(handle.poll_events());
        assert_eq!(
            events,
            [EngineEvent::InternalError {
                detail: "impossible state in an audio thread".to_string()
            }]
        );
    }
}
