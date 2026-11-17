//! Everything that runs on the output audio thread (06 §2). RT-01 to RT-10 apply (06 §4).

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub(crate) mod commands;
pub(crate) mod master;
pub(crate) mod metronome;
pub(crate) mod processor;
pub(crate) mod sampler;
pub(crate) mod sequencer;
pub(crate) mod strip;
pub(crate) mod transport;

/// Largest subblock, in frames (06 §19).
pub const MAX_BLOCK: usize = 4_096;
