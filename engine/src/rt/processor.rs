//! The `Processor`: computes each block (06 §7). The output thread and the offline render use
//! the same one (06 §1).

use std::sync::Arc;

use crate::rt::commands::{CommandConsumer, RtCommand, COMMAND_QUEUE_CAPACITY};
use crate::rt::master::Master;
use crate::rt::metronome::TestTone;
use crate::rt::MAX_BLOCK;
use crate::telemetry::{PublishedTransport, SharedState};

/// Computes the engine's output block by block. It is created in `Stopped`, with the engine
/// clock at 0 (06 §13.2).
pub struct Processor {
    commands: CommandConsumer,
    shared: Arc<SharedState>,
    master: Master,
    test_tone: TestTone,
    /// Frames computed since the `Processor` was created (06 §6.2).
    engine_clock: u64,
}

impl Processor {
    pub(crate) fn new(commands: CommandConsumer, shared: Arc<SharedState>) -> Self {
        Self {
            commands,
            shared,
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

        // 2. Compute the block by subblocks of at most MAX_BLOCK frames (06 §6.4).
        for subblock in out.chunks_mut(2 * MAX_BLOCK) {
            let frames = subblock.len() / 2;
            self.master.clear(frames);
            // TODO(T-007): the source of each track through its channel (06 §7, step 2.2).
            // TODO(T-047): the metronome and count-in clicks (06 §7, step 2.3).
            self.master.finish(&mut self.test_tone, subblock);
            // TODO(T-007): advance the position and run the event that ends the subblock.
            self.engine_clock += frames as u64;
        }

        // RT-09: only finite samples leave the engine.
        if out.iter().any(|sample| !sample.is_finite()) {
            out.fill(0.0);
            self.shared.raise_internal_error();
        }

        // 3. Publish the atomic values of the telemetry (06 §18.1).
        // TODO(T-007): the state, mode, position and step of the transport.
        self.shared
            .publish_transport(PublishedTransport::Stopped, 0, -1);
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
                RtCommand::TestTone => self.test_tone.start(),
                // TODO(T-007): adopt the snapshot, which goes to the trash queue when it is no
                // longer used (TODO(T-033)); play, stop and audition (06 §6.3, §8.3).
                RtCommand::SetProject(_)
                | RtCommand::Play(_)
                | RtCommand::Stop
                | RtCommand::Audition { .. } => {}
                // TODO(T-046): seek and loop (06 §6.3, §6.5).
                RtCommand::Seek { .. } | RtCommand::SetLoop { .. } => {}
                // TODO(T-047): metronome and count-in (06 §6.3, §12).
                RtCommand::SetMetronome { .. } | RtCommand::RecordStart { .. } => {}
                // TODO(T-085): calibration signal (06 §15).
                RtCommand::Calibration(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rt::commands::command_queue;

    #[test]
    fn an_output_shorter_than_the_block_is_silence_and_an_internal_error() {
        let (_commands, consumer) = command_queue();
        let shared = Arc::new(SharedState::new());
        let mut processor = Processor::new(consumer, Arc::clone(&shared));
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
        let mut processor = Processor::new(consumer, Arc::clone(&shared));
        let mut out = [0.5_f32; 6];
        processor.process(&mut out, 2);
        assert_eq!(out, [0.0, 0.0, 0.0, 0.0, 0.5, 0.5]);
        assert!(!shared.take_internal_error());
        assert_eq!(shared.engine_clock(), 2);
    }
}
