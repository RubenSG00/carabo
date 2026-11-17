//! The `Processor`: computes each block (06 §7). The output thread and the offline render use
//! the same one (06 §1).

use std::sync::Arc;

use crate::constants::{LEVEL_GAINS, MAX_TRACKS};
use crate::kits::Kits;
use crate::project::EngineProject;
use crate::rt::commands::{CommandConsumer, RtCommand, COMMAND_QUEUE_CAPACITY};
use crate::rt::master::Master;
use crate::rt::metronome::TestTone;
use crate::rt::sampler::Voices;
use crate::rt::transport::{Change, Running, Transport};
use crate::rt::{sequencer, strip, MAX_BLOCK};
use crate::telemetry::SharedState;

/// Level with which an audition sounds: 2, normal (06 §8.3).
const AUDITION_LEVEL: usize = 2;

/// Computes the engine's output block by block. It is created in `Stopped`, with the engine
/// clock at 0 (06 §13.2).
pub struct Processor {
    commands: CommandConsumer,
    shared: Arc<SharedState>,
    /// The three kits, loaded by `Engine::start` (06 §8.4).
    kits: Arc<Kits>,
    /// The prepared snapshot in use; `None` until the first one arrives.
    project: Option<Arc<EngineProject>>,
    transport: Transport,
    /// The voices of each slot (06 §5.4).
    slots: Box<[Voices]>,
    /// Interleaved stereo buffer of a track: `2 · MAX_BLOCK` samples (06 §19).
    track_bus: Box<[f32]>,
    master: Master,
    test_tone: TestTone,
    /// Frames computed since the `Processor` was created (06 §6.2).
    engine_clock: u64,
}

impl Processor {
    pub(crate) fn new(
        commands: CommandConsumer,
        shared: Arc<SharedState>,
        kits: Arc<Kits>,
    ) -> Self {
        Self {
            commands,
            shared,
            kits,
            project: None,
            transport: Transport::new(),
            slots: vec![Voices::default(); MAX_TRACKS].into_boxed_slice(),
            track_bus: vec![0.0; 2 * MAX_BLOCK].into_boxed_slice(),
            master: Master::new(),
            test_tone: TestTone::default(),
            engine_clock: 0,
        }
    }

    /// Writes `n` interleaved stereo frames (`L, R, L, R…`) to the start of `out` (06 §7).
    /// `out` must hold at least `2 · n` samples.
    pub fn process(&mut self, out: &mut [f32], n: usize) {
        let Some(out) = n.checked_mul(2).and_then(|len| out.get_mut(..len)) else {
            // 06 §20: an impossible state. The block is delivered as silence.
            out.fill(0.0);
            self.shared.raise_internal_error();
            return;
        };

        // 1. Apply the commands that have arrived since the previous block.
        self.apply_commands();

        // 2. Compute the block by subblocks (06 §6.4), at most MAX_BLOCK frames each.
        let mut impossible = false;
        for chunk in out.chunks_mut(2 * MAX_BLOCK) {
            let frames = chunk.len() / 2;
            let mut done = 0;
            // RT-07: every subblock has at least one frame.
            for _ in 0..MAX_BLOCK {
                if done == frames {
                    break;
                }
                let left = (frames - done) as u64;
                let m = self.transport.next_subblock(left, self.project.as_deref()) as usize;
                let Some(subblock) = chunk.get_mut(2 * done..2 * (done + m)).filter(|_| m > 0)
                else {
                    break;
                };
                impossible |= self.compute_subblock(subblock, m);
                self.engine_clock += m as u64;
                if self.transport.advance(m as u64, self.project.as_deref()) == Change::Stopped {
                    self.silence();
                }
                done += m;
            }
            if done < frames {
                impossible = true;
                self.engine_clock += (frames - done) as u64;
            }
        }

        // RT-09: only finite samples leave the engine.
        if impossible || out.iter().any(|sample| !sample.is_finite()) {
            out.fill(0.0);
            self.shared.raise_internal_error();
        }

        // 3. Publish the atomic values of the telemetry (06 §18.1).
        let (transport, position, step) = self.transport.published(self.project.as_deref());
        self.shared.publish_transport(transport, position, step);
        self.shared.set_engine_clock(self.engine_clock);
    }

    /// Samples clipped by the master since the `Processor` was created (06 §10.3).
    pub fn clipped_samples(&self) -> u64 {
        self.master.clipped_samples()
    }

    /// Empties the command queue and applies each command in order (06 §5.2). RT-07: at most
    /// the capacity of the queue per block.
    fn apply_commands(&mut self) {
        for _ in 0..COMMAND_QUEUE_CAPACITY {
            let Ok(command) = self.commands.pop() else {
                break;
            };
            match command {
                // The control thread keeps a reference to every project it sends, so the one
                // replaced here is not freed on this thread (RT-02).
                // TODO(T-033): adopt the snapshot as 06 §6.6 says and send the previous one to
                // the trash queue (06 §5.6).
                RtCommand::SetProject(project) => self.project = Some(project),
                RtCommand::Play(mode) => {
                    if self.transport.play(mode) == Change::Stopped {
                        self.silence();
                    }
                }
                RtCommand::Stop => self.transport.stop(),
                RtCommand::Audition { slot, row } => self.audition(slot, row),
                RtCommand::TestTone => self.test_tone.start(),
                // TODO(T-046): seek and loop (06 §6.3, §6.5).
                RtCommand::Seek { .. } | RtCommand::SetLoop { .. } => {}
                // TODO(T-047): metronome and count-in (06 §6.3, §12).
                RtCommand::SetMetronome { .. } | RtCommand::RecordStart { .. } => {}
                // TODO(T-085): calibration signal (06 §15).
                RtCommand::Calibration(_) => {}
            }
        }
    }

    /// Computes a subblock of `m` frames into `out` (06 §7, steps 2.1 to 2.5). Returns whether
    /// it has met an impossible state (06 §20).
    fn compute_subblock(&mut self, out: &mut [f32], m: usize) -> bool {
        let Self {
            kits,
            project,
            transport,
            slots,
            track_bus,
            master,
            test_tone,
            ..
        } = self;
        let mut impossible = false;
        let bus = master.clear(m);
        let running = transport.running();
        let position = transport.position();

        // Each rhythm track: its source through its channel, added to the master (06 §7, §8).
        for track in project.iter().flat_map(|project| &project.rhythm_tracks) {
            let (Some(voices), Some(source)) =
                (slots.get_mut(track.slot), track_bus.get_mut(..2 * m))
            else {
                impossible = true;
                continue;
            };
            source.fill(0.0);
            // In pattern mode, the chosen track is sequenced and audible, and the others only
            // sound their voices already fired, with `audible` 0 (06 §7).
            let (pattern, audible) = match running {
                None => (None, 1.0),
                Some(Running::Pattern {
                    slot,
                    pattern_index,
                }) if slot == track.slot => (track.patterns.get(pattern_index), 1.0),
                Some(Running::Pattern { .. }) => (None, 0.0),
            };
            match (pattern, project.as_deref()) {
                (Some(pattern), Some(project)) => sequencer::play_pattern(
                    track,
                    pattern,
                    project.tempo_bpm,
                    position,
                    voices,
                    kits,
                    source,
                ),
                _ => voices.render(kits, source),
            }
            strip::mix(&track.strip, audible, source, bus);
        }

        // TODO(T-047): the metronome and count-in clicks (06 §7, step 2.3).
        master.finish(test_tone, transport.stop_fade(), out);
        impossible
    }

    /// `Audition { slot, row }` (06 §8.3): fires the sound of the row with delay 0 and gain
    /// `LEVEL_GAINS[2] · rowGain`, even if the row is muted.
    // TODO(T-047): ignored in `CountIn`.
    fn audition(&mut self, slot: usize, row: usize) {
        let Some(track) = self.project.as_deref().and_then(|p| p.rhythm_track(slot)) else {
            return;
        };
        let (Some(voices), Some(settings)) = (self.slots.get_mut(slot), track.rows.get(row)) else {
            return;
        };
        let gain = LEVEL_GAINS[AUDITION_LEVEL] * settings.gain;
        voices.fire(&self.kits, track.kit, row, gain);
    }

    /// On reaching `Stopped`, every voice is switched off (06 §6.1).
    // TODO(T-041, T-042): and the memory of the filters and reverbs of every slot is cleared.
    fn silence(&mut self) {
        for voices in self.slots.iter_mut() {
            voices.silence();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::KIT_ROWS;
    use crate::kits::{Kit, KitSound};
    use crate::rt::commands::command_queue;

    fn kits() -> Arc<Kits> {
        let kit = Kit {
            sounds: std::array::from_fn(|_| KitSound {
                samples: vec![1.0],
                choke_group: 0,
            }),
        };
        Arc::new(Kits {
            kits: [kit.clone(), kit.clone(), kit],
        })
    }

    #[test]
    fn an_output_shorter_than_the_block_is_silence_and_an_internal_error() {
        let (_commands, consumer) = command_queue();
        let shared = Arc::new(SharedState::new());
        let mut processor = Processor::new(consumer, Arc::clone(&shared), kits());
        let mut out = [0.5_f32; 6];
        processor.process(&mut out, 4);
        assert_eq!(out, [0.0; 6]);
        assert!(shared.take_internal_error());
        assert_eq!(shared.engine_clock(), 0);
    }

    #[test]
    fn a_longer_output_keeps_what_follows_the_block() {
        let (_commands, consumer) = command_queue();
        let shared = Arc::new(SharedState::new());
        let mut processor = Processor::new(consumer, Arc::clone(&shared), kits());
        let mut out = [0.5_f32; 6];
        processor.process(&mut out, 2);
        assert_eq!(out, [0.0, 0.0, 0.0, 0.0, 0.5, 0.5]);
        assert!(!shared.take_internal_error());
        assert_eq!(shared.engine_clock(), 2);
    }

    #[test]
    fn an_audition_without_a_project_or_outside_the_rows_does_nothing() {
        let (mut commands, consumer) = command_queue();
        let shared = Arc::new(SharedState::new());
        let mut processor = Processor::new(consumer, Arc::clone(&shared), kits());
        assert!(commands
            .push(RtCommand::Audition { slot: 0, row: 0 })
            .is_ok());
        assert!(commands
            .push(RtCommand::Audition {
                slot: MAX_TRACKS,
                row: KIT_ROWS
            })
            .is_ok());
        let mut out = [0.5_f32; 20];
        processor.process(&mut out, 10);
        assert_eq!(out, [0.0; 20]);
        assert!(!shared.take_internal_error());
    }
}
