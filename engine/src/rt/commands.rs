//! Commands from the control thread to the output thread (06 §5.2), through a non-blocking
//! single-producer single-consumer queue.

use std::sync::Arc;

use rtrb::RingBuffer;

use crate::project::EngineProject;

/// Capacity of the command queue (06 §19).
pub(crate) const COMMAND_QUEUE_CAPACITY: usize = 256;

/// End of the command queue that the control thread writes.
pub type CommandProducer = rtrb::Producer<RtCommand>;
/// End of the command queue that the output thread reads.
pub(crate) type CommandConsumer = rtrb::Consumer<RtCommand>;

/// A command for the output thread (06 §5.2).
#[derive(Debug)]
pub enum RtCommand {
    /// A prepared snapshot (06 §5.3).
    SetProject(Arc<EngineProject>),
    Play(PlayMode),
    Stop,
    Seek {
        frame: u64,
    },
    SetLoop {
        enabled: bool,
        start_frame: u64,
        end_frame: u64,
    },
    SetMetronome {
        enabled: bool,
    },
    Audition {
        slot: usize,
        row: usize,
    },
    RecordStart {
        slot: usize,
        start_frame: u64,
    },
    TestTone,
    /// The signal to emit while calibrating (06 §15).
    Calibration(Arc<[f32]>),
}

/// What `RtCommand::Play` plays (06 §5.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    Song { start_frame: u64 },
    Pattern { slot: usize, pattern_index: usize },
}

/// A new, empty command queue.
pub(crate) fn command_queue() -> (CommandProducer, CommandConsumer) {
    RingBuffer::new(COMMAND_QUEUE_CAPACITY)
}
