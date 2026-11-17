//! What the engine tells the outside world (06 §18): the atomic values that the audio threads
//! publish, the `Telemetry` built from them (04 §6.4) and the events of the control thread.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicU8, Ordering};

use serde::Serialize;

use crate::constants::MAX_TRACKS;
use crate::io::devices::ActiveDevices;

/// What the engine reports, `TELEMETRY_HZ` times per second, through `engine://telemetry`
/// (04 §6.4).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Telemetry {
    pub state: TelemetryState,
    /// `None` when stopped.
    pub mode: Option<TelemetryMode>,
    /// Song mode: frame of the song; pattern mode: frame from the start of the pattern.
    pub position_frame: u64,
    /// Pattern mode: the step that is sounding.
    pub pattern_step: Option<u32>,
    /// For each track id: left and right peak since the previous reading, as linear amplitude.
    pub track_peaks: BTreeMap<String, [f32; 2]>,
    pub master_peak: [f32; 2],
    /// `None` while the input is closed.
    pub input_peak: Option<f32>,
    /// Audio dropouts since the last playback started.
    pub xruns: u32,
    /// Frames recorded so far, or `None` when not recording.
    pub recorded_frames: Option<u64>,
    /// `false` when there is no output device.
    pub output_available: bool,
}

/// The transport state as the interface sees it. `Stopping` is reported as `Stopped` (06 §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TelemetryState {
    Stopped,
    CountIn,
    Playing,
    Recording,
}

/// The play mode as the interface sees it (04 §6.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TelemetryMode {
    Song,
    Pattern,
}

/// What the control thread reports through `EngineHandle::poll_events()` (06 §18.2).
#[derive(Debug, Clone, PartialEq)]
pub enum EngineEvent {
    /// A recording has finished, or its count-in has been cancelled.
    RecordingFinished {
        file: String,
        frames: u64,
        track_id: String,
        start_frame: u64,
        reason: RecordEndReason,
    },
    /// A device in use has disappeared.
    DevicesChanged {
        lost_output_name: Option<String>,
        lost_input_name: Option<String>,
        active: ActiveDevices,
    },
    /// Audio files of the snapshot that could not be loaded: the `file` text of each, as it came
    /// in the snapshot (06 §9.1).
    AudioUnavailable { files: Vec<String> },
    /// An internal error of the engine (06 §20).
    InternalError { detail: String },
}

/// Why a recording finished (04 §6.4, `RecordingFinished.reason`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordEndReason {
    Stopped,
    Cancelled,
    SongLimit,
    InputLost,
    OutputLost,
    DiskError,
}

/// State and mode of the transport in one byte, as the output thread publishes them (06 §18.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedTransport {
    Stopped = 0,
    CountIn = 1,
    PlayingSong = 2,
    PlayingPattern = 3,
    Recording = 4,
}

impl PublishedTransport {
    fn from_code(code: u8) -> Self {
        match code {
            1 => Self::CountIn,
            2 => Self::PlayingSong,
            3 => Self::PlayingPattern,
            4 => Self::Recording,
            _ => Self::Stopped,
        }
    }

    /// State and mode for the telemetry: `None` when stopped and `song` in `CountIn` and
    /// `Recording` (06 §18.1).
    fn state_and_mode(self) -> (TelemetryState, Option<TelemetryMode>) {
        match self {
            Self::Stopped => (TelemetryState::Stopped, None),
            Self::CountIn => (TelemetryState::CountIn, Some(TelemetryMode::Song)),
            Self::PlayingSong => (TelemetryState::Playing, Some(TelemetryMode::Song)),
            Self::PlayingPattern => (TelemetryState::Playing, Some(TelemetryMode::Pattern)),
            Self::Recording => (TelemetryState::Recording, Some(TelemetryMode::Song)),
        }
    }
}

/// The atomic values shared by the engine's threads (06 §18.1). The audio threads only write
/// them with atomic operations; `EngineHandle::telemetry()` reads them without going through
/// the control thread.
///
/// Peaks hold the bits of a non-negative `f32`, whose order is that of the bits read as an
/// integer: writers use `fetch_max` and every reading replaces them with 0, so each reading
/// returns the maximum since the previous one.
#[derive(Debug, Default)]
pub struct SharedState {
    transport: AtomicU8,
    position: AtomicU64,
    /// −1 when it does not apply.
    pattern_step: AtomicI32,
    slot_peaks: [[AtomicU32; 2]; MAX_TRACKS],
    master_peak: [AtomicU32; 2],
    input_peak: AtomicU32,
    xruns: AtomicU32,
    recorded_frames: AtomicU64,
    engine_clock: AtomicU64,
    internal_error: AtomicBool,
    output_available: AtomicBool,
    input_open: AtomicBool,
}

impl SharedState {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Output thread: the state of the transport after a block (06 §7, step 3).
    pub(crate) fn publish_transport(
        &self,
        transport: PublishedTransport,
        position: u64,
        pattern_step: i32,
    ) {
        self.position.store(position, Ordering::Relaxed);
        self.pattern_step.store(pattern_step, Ordering::Relaxed);
        self.transport.store(transport as u8, Ordering::Release);
    }

    /// Output thread: the engine clock after a block (06 §6.2).
    pub(crate) fn set_engine_clock(&self, frames: u64) {
        self.engine_clock.store(frames, Ordering::Release);
    }

    /// The engine clock: frames computed by the output thread since it started (06 §6.2).
    pub fn engine_clock(&self) -> u64 {
        self.engine_clock.load(Ordering::Acquire)
    }

    /// Audio threads: an impossible state has been met (06 §20).
    pub(crate) fn raise_internal_error(&self) {
        self.internal_error.store(true, Ordering::Release);
    }

    /// Control thread: whether an audio thread has raised the internal error flag since the
    /// last call. Clears it, so that each incident is reported once (06 §20).
    pub(crate) fn take_internal_error(&self) -> bool {
        self.internal_error.swap(false, Ordering::AcqRel)
    }

    /// Control thread: whether there is an output stream.
    pub(crate) fn set_output_available(&self, available: bool) {
        self.output_available.store(available, Ordering::Release);
    }

    /// Builds a `Telemetry` (06 §18.1). `track_slots` gives the slot of each track of the last
    /// snapshot, in the order of the snapshot.
    pub fn telemetry(&self, track_slots: &[(String, usize)]) -> Telemetry {
        let transport = PublishedTransport::from_code(self.transport.load(Ordering::Acquire));
        let (state, mode) = transport.state_and_mode();
        let pattern_step = match transport {
            PublishedTransport::PlayingPattern => {
                u32::try_from(self.pattern_step.load(Ordering::Relaxed)).ok()
            }
            _ => None,
        };
        let track_peaks = track_slots
            .iter()
            .filter_map(|(id, slot)| Some((id.clone(), take_peaks(self.slot_peaks.get(*slot)?))))
            .collect();
        let input_peak = self
            .input_open
            .load(Ordering::Acquire)
            .then(|| take_peak(&self.input_peak));
        let recorded_frames = (transport == PublishedTransport::Recording)
            .then(|| self.recorded_frames.load(Ordering::Relaxed));
        Telemetry {
            state,
            mode,
            position_frame: self.position.load(Ordering::Relaxed),
            pattern_step,
            track_peaks,
            master_peak: take_peaks(&self.master_peak),
            input_peak,
            xruns: self.xruns.load(Ordering::Relaxed),
            recorded_frames,
            output_available: self.output_available.load(Ordering::Acquire),
        }
    }
}

/// Reads a peak and leaves 0 in its place.
fn take_peak(peak: &AtomicU32) -> f32 {
    f32::from_bits(peak.swap(0, Ordering::Relaxed))
}

fn take_peaks(peaks: &[AtomicU32; 2]) -> [f32; 2] {
    [take_peak(&peaks[0]), take_peak(&peaks[1])]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn telemetry_with(transport: PublishedTransport) -> Telemetry {
        let shared = SharedState::new();
        shared.publish_transport(transport, 1_234, 5);
        shared.telemetry(&[])
    }

    #[test]
    fn stopping_and_each_state_give_the_state_and_mode_of_06_18_1() {
        let cases = [
            (
                PublishedTransport::Stopped,
                TelemetryState::Stopped,
                None,
                None,
            ),
            (
                PublishedTransport::CountIn,
                TelemetryState::CountIn,
                Some(TelemetryMode::Song),
                None,
            ),
            (
                PublishedTransport::PlayingSong,
                TelemetryState::Playing,
                Some(TelemetryMode::Song),
                None,
            ),
            (
                PublishedTransport::PlayingPattern,
                TelemetryState::Playing,
                Some(TelemetryMode::Pattern),
                Some(5),
            ),
            (
                PublishedTransport::Recording,
                TelemetryState::Recording,
                Some(TelemetryMode::Song),
                None,
            ),
        ];
        for (transport, state, mode, step) in cases {
            let t = telemetry_with(transport);
            assert_eq!((t.state, t.mode, t.pattern_step), (state, mode, step));
            assert_eq!(t.position_frame, 1_234);
        }
    }

    #[test]
    fn recorded_frames_only_while_recording() {
        let shared = SharedState::new();
        shared.recorded_frames.store(96_000, Ordering::Relaxed);
        assert_eq!(shared.telemetry(&[]).recorded_frames, None);
        shared.publish_transport(PublishedTransport::Recording, 0, -1);
        assert_eq!(shared.telemetry(&[]).recorded_frames, Some(96_000));
    }

    #[test]
    fn input_peak_only_while_the_input_is_open() {
        let shared = SharedState::new();
        shared
            .input_peak
            .store(0.5_f32.to_bits(), Ordering::Relaxed);
        assert_eq!(shared.telemetry(&[]).input_peak, None);
        shared.input_open.store(true, Ordering::Relaxed);
        assert_eq!(shared.telemetry(&[]).input_peak, Some(0.5));
        assert_eq!(shared.telemetry(&[]).input_peak, Some(0.0));
    }

    #[test]
    fn each_reading_of_a_peak_returns_the_maximum_since_the_previous_one() {
        let shared = SharedState::new();
        for value in [0.25_f32, 0.75, 0.5] {
            shared.master_peak[0].fetch_max(value.to_bits(), Ordering::Relaxed);
        }
        shared.slot_peaks[3][1].fetch_max(0.125_f32.to_bits(), Ordering::Relaxed);
        let tracks = [("drums".to_string(), 3), ("voice".to_string(), 0)];

        let first = shared.telemetry(&tracks);
        assert_eq!(first.master_peak, [0.75, 0.0]);
        assert_eq!(first.track_peaks["drums"], [0.0, 0.125]);
        assert_eq!(first.track_peaks["voice"], [0.0, 0.0]);

        let second = shared.telemetry(&tracks);
        assert_eq!(second.master_peak, [0.0, 0.0]);
        assert_eq!(second.track_peaks["drums"], [0.0, 0.0]);
    }

    #[test]
    fn the_internal_error_flag_is_reported_once_per_incident() {
        let shared = SharedState::new();
        assert!(!shared.take_internal_error());
        shared.raise_internal_error();
        assert!(shared.take_internal_error());
        assert!(!shared.take_internal_error());
    }

    #[test]
    fn telemetry_serializes_with_the_keys_and_values_of_04_6_4() {
        let telemetry = Telemetry {
            state: TelemetryState::CountIn,
            mode: Some(TelemetryMode::Song),
            position_frame: 96_000,
            pattern_step: None,
            track_peaks: BTreeMap::from([("t1".to_string(), [0.5, 0.25])]),
            master_peak: [0.5, 0.25],
            input_peak: None,
            xruns: 0,
            recorded_frames: None,
            output_available: true,
        };
        assert_eq!(
            serde_json::to_value(&telemetry).expect("a telemetry serializes"),
            json!({
                "state": "countIn",
                "mode": "song",
                "positionFrame": 96000,
                "patternStep": null,
                "trackPeaks": { "t1": [0.5, 0.25] },
                "masterPeak": [0.5, 0.25],
                "inputPeak": null,
                "xruns": 0,
                "recordedFrames": null,
                "outputAvailable": true
            })
        );
    }

    #[test]
    fn states_modes_and_end_reasons_serialize_as_in_04_6_4() {
        let text = |value: serde_json::Value| value.as_str().unwrap_or_default().to_string();
        let states = [
            TelemetryState::Stopped,
            TelemetryState::CountIn,
            TelemetryState::Playing,
            TelemetryState::Recording,
        ]
        .map(|s| text(json!(s)));
        assert_eq!(states, ["stopped", "countIn", "playing", "recording"]);
        let modes = [TelemetryMode::Song, TelemetryMode::Pattern].map(|m| text(json!(m)));
        assert_eq!(modes, ["song", "pattern"]);
        let reasons = [
            RecordEndReason::Stopped,
            RecordEndReason::Cancelled,
            RecordEndReason::SongLimit,
            RecordEndReason::InputLost,
            RecordEndReason::OutputLost,
            RecordEndReason::DiskError,
        ]
        .map(|r| text(json!(r)));
        assert_eq!(
            reasons,
            [
                "stopped",
                "cancelled",
                "songLimit",
                "inputLost",
                "outputLost",
                "diskError"
            ]
        );
    }
}
