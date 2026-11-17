//! The master channel (06 §10.3).

use crate::rt::metronome::TestTone;
use crate::rt::MAX_BLOCK;

/// The master channel: the bus where tracks and clicks are summed, and what is applied to it
/// frame by frame before it is delivered (06 §10.3).
#[derive(Debug)]
pub(crate) struct Master {
    /// Interleaved stereo bus of `2 · MAX_BLOCK` samples, reserved when it is created (06 §19).
    bus: Box<[f32]>,
    /// Samples clipped since it was created; the export returns them as `clipped_samples`.
    clipped_samples: u64,
}

impl Master {
    pub(crate) fn new() -> Self {
        Self {
            bus: vec![0.0; 2 * MAX_BLOCK].into_boxed_slice(),
            clipped_samples: 0,
        }
    }

    /// Clears the bus for a subblock of `frames` frames, at most `MAX_BLOCK`, and returns it
    /// (06 §7, step 2.1).
    pub(crate) fn clear(&mut self, frames: usize) -> &mut [f32] {
        let len = (2 * frames).min(self.bus.len());
        let bus = &mut self.bus[..len];
        bus.fill(0.0);
        bus
    }

    /// Applies the master to the bus and writes the result to `out`, interleaved stereo of the
    /// same length as the cleared bus (06 §7, step 2.4):
    ///
    /// ```text
    /// x = bus + test tone;   if |x| > 1: clipped += 1, x = sign(x) · 1.0
    /// ```
    // TODO(T-040): the master volume with its ramp, and the master meter (06 §10.3).
    // TODO(T-007): the stop ramp of `Stopping` (06 §10.3).
    // TODO(T-085): the calibration signal, added like the test tone (06 §10.3, §15).
    pub(crate) fn finish(&mut self, test_tone: &mut TestTone, out: &mut [f32]) {
        let (out_frames, _) = out.as_chunks_mut::<2>();
        let (bus_frames, _) = self.bus.as_chunks::<2>();
        for (out_frame, bus_frame) in out_frames.iter_mut().zip(bus_frames) {
            let tone = test_tone.next_sample();
            for (sample, sum) in out_frame.iter_mut().zip(bus_frame) {
                *sample = clip(sum + tone, &mut self.clipped_samples);
            }
        }
    }

    /// Samples clipped since the master was created (RF-603).
    pub(crate) fn clipped_samples(&self) -> u64 {
        self.clipped_samples
    }
}

/// Limits `x` to ±1,0, counting it in `clipped` if it goes beyond.
fn clip(x: f32, clipped: &mut u64) -> f32 {
    if x.abs() > 1.0 {
        *clipped += 1;
        x.signum()
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::TAU;

    /// Two tones of amplitude 0,75 at 440 and 660 Hz: their sum goes beyond ±1,0.
    fn two_tones(frame: usize, channel: usize) -> f32 {
        let t = frame as f32 / 48_000.0;
        let sum = 0.75 * (TAU * 440.0 * t).sin() + 0.75 * (TAU * 660.0 * t).sin();
        if channel == 0 {
            sum
        } else {
            -sum
        }
    }

    #[test]
    fn the_master_clips_to_one_and_counts_the_clipped_samples() {
        let mut master = Master::new();
        let mut tone = TestTone::default();
        let mut out = vec![0.0; 2 * MAX_BLOCK];
        let mut expected_clipped = 0;

        for block in 0..3 {
            let bus = master.clear(MAX_BLOCK);
            for (i, sample) in bus.iter_mut().enumerate() {
                *sample = two_tones(block * MAX_BLOCK + i / 2, i % 2);
            }
            master.finish(&mut tone, &mut out);

            for (i, sample) in out.iter().enumerate() {
                let x = two_tones(block * MAX_BLOCK + i / 2, i % 2);
                if x.abs() > 1.0 {
                    expected_clipped += 1;
                    assert_eq!(*sample, x.signum(), "sample {i} of block {block}");
                } else {
                    assert_eq!(*sample, x, "sample {i} of block {block}");
                }
            }
        }
        assert!(expected_clipped > 1_000, "the two tones go beyond ±1,0");
        assert_eq!(master.clipped_samples(), expected_clipped);
        assert!(out.iter().all(|s| s.abs() <= 1.0));
    }

    #[test]
    fn a_signal_within_one_is_not_touched_nor_counted() {
        let mut master = Master::new();
        let values = [1.0, -1.0, 0.5, -0.999_999];
        master.clear(2).copy_from_slice(&values);
        let mut out = [0.0; 4];
        master.finish(&mut TestTone::default(), &mut out);
        assert_eq!(out, values);
        assert_eq!(master.clipped_samples(), 0);
    }

    #[test]
    fn the_test_tone_is_added_to_both_channels_and_clipped_too() {
        let mut master = Master::new();
        let mut tone = TestTone::default();
        tone.start();
        // Advance to the top of the tone: frame 1 227 is just past a quarter period
        // (1 200 + 27,27), where the tone is about +0,25.
        for _ in 0..1_227 {
            tone.next_sample();
        }
        master.clear(1).copy_from_slice(&[0.9, 0.0]);
        let mut out = [0.0; 2];
        master.finish(&mut tone, &mut out);
        assert_eq!(out[0], 1.0);
        assert!((out[1] - 0.25).abs() < 0.001, "{}", out[1]);
        assert_eq!(master.clipped_samples(), 1);
    }
}
