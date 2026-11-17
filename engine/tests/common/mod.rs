//! Shared helpers for the engine's integration tests (08 §3.4, §8.1): snapshot builders and the
//! harness that drives a `Processor` without an audio device.

// Each test file uses only part of this module.
#![allow(dead_code)]

use std::ops::Range;
use std::sync::Arc;

use carabo_engine::snapshot::{EngineSnapshot, MasterStrip, SNAPSHOT_VERSION};
use carabo_engine::testing::{self, CommandProducer, Processor, RtCommand, SharedState};
use carabo_engine::Telemetry;

/// Builder of snapshots for the tests (08 §3.4).
pub struct SnapshotBuilder {
    snapshot: EngineSnapshot,
}

/// A valid snapshot without tracks: 120, 4/4 and the master at 0 dB.
pub fn snapshot() -> SnapshotBuilder {
    SnapshotBuilder {
        snapshot: EngineSnapshot {
            snapshot_version: SNAPSHOT_VERSION,
            tempo_bpm: 120.0,
            time_signature: "4/4".to_string(),
            tracks: Vec::new(),
            master: MasterStrip { gain_db: 0.0 },
        },
    }
}

impl SnapshotBuilder {
    pub fn build(self) -> EngineSnapshot {
        self.snapshot
    }
}

/// Drives a `Processor` directly, as the output thread does (08 §8.1).
pub struct Harness {
    processor: Processor,
    commands: CommandProducer,
    shared: Arc<SharedState>,
}

impl Harness {
    /// Creates a `Processor` in `Stopped` for `snapshot`.
    pub fn new(snapshot: &EngineSnapshot) -> Self {
        snapshot
            .validate()
            .expect("the tests only use valid snapshots");
        // TODO(T-007): prepare the snapshot (06 §5.3) and send it with `RtCommand::SetProject`.
        let (processor, commands, shared) = testing::processor();
        Self {
            processor,
            commands,
            shared,
        }
    }

    /// Queues a command; the `Processor` applies it at the start of the next block (06 §7).
    pub fn send(&mut self, command: RtCommand) {
        assert!(
            self.commands.push(command).is_ok(),
            "the command queue is full"
        );
    }

    /// Computes `frames` frames in blocks of `block` frames (the last one, of what is left) and
    /// returns them as interleaved stereo.
    pub fn run(&mut self, frames: usize, block: usize) -> Vec<f32> {
        assert!(block > 0);
        let mut out = vec![0.0; 2 * frames];
        let mut done = 0;
        while done < frames {
            let n = block.min(frames - done);
            self.processor
                .process(&mut out[2 * done..2 * (done + n)], n);
            done += n;
        }
        out
    }

    /// The telemetry as `EngineHandle::telemetry()` would build it.
    pub fn telemetry(&self) -> Telemetry {
        self.shared.telemetry(&[])
    }

    /// The engine clock (06 §6.2).
    pub fn engine_clock(&self) -> u64 {
        self.shared.engine_clock()
    }

    /// Samples clipped by the master since the `Processor` was created (06 §10.3).
    pub fn clipped_samples(&self) -> u64 {
        self.processor.clipped_samples()
    }
}

/// Sample of `channel` (0 left, 1 right) at `frame` of an interleaved stereo buffer.
pub fn sample(out: &[f32], channel: usize, frame: usize) -> f32 {
    out[2 * frame + channel]
}

/// Number of frames of an interleaved stereo buffer.
pub fn frames(out: &[f32]) -> usize {
    out.len() / 2
}

/// First frame at or after `from` where either channel is not 0.
pub fn first_nonzero_frame(out: &[f32], from: usize) -> Option<usize> {
    (from..frames(out)).find(|&f| sample(out, 0, f) != 0.0 || sample(out, 1, f) != 0.0)
}

/// Root mean square of `channel` over `range` of frames.
pub fn rms(out: &[f32], channel: usize, range: Range<usize>) -> f64 {
    let n = range.len() as f64;
    let sum: f64 = range
        .map(|f| f64::from(sample(out, channel, f)).powi(2))
        .sum();
    (sum / n).sqrt()
}

/// Largest absolute value of `channel` over `range` of frames.
pub fn peak(out: &[f32], channel: usize, range: Range<usize>) -> f32 {
    range
        .map(|f| sample(out, channel, f).abs())
        .fold(0.0, f32::max)
}

/// Amplitude of a tone of `hz` in `channel` over `range` of frames, by correlation with a sine
/// and a cosine. Exact for a whole number of periods.
pub fn tone_level(out: &[f32], hz: f64, channel: usize, range: Range<usize>) -> f64 {
    let n = range.len() as f64;
    let (mut sin_sum, mut cos_sum) = (0.0, 0.0);
    for f in range {
        let x = f64::from(sample(out, channel, f));
        let phase = 2.0 * std::f64::consts::PI * hz * f as f64 / 48_000.0;
        sin_sum += x * phase.sin();
        cos_sum += x * phase.cos();
    }
    2.0 * (sin_sum * sin_sum + cos_sum * cos_sum).sqrt() / n
}

/// Largest difference between two consecutive samples of the same channel over `range` of
/// frames.
pub fn max_step(out: &[f32], range: Range<usize>) -> f32 {
    let mut largest: f32 = 0.0;
    for f in range.start + 1..range.end {
        for channel in 0..2 {
            let step = (sample(out, channel, f) - sample(out, channel, f - 1)).abs();
            largest = largest.max(step);
        }
    }
    largest
}
