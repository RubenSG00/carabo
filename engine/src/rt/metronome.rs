//! The test tone (06 §12, RF-700).
// TODO(T-047): the metronome and count-in clicks (06 §12).

use std::f64::consts::TAU;

use crate::constants::{MICROFADE_FRAMES, SAMPLE_RATE};

/// Length of the test tone: one second (06 §12).
const TEST_TONE_FRAMES: u32 = SAMPLE_RATE;
const TEST_TONE_HZ: u64 = 440;
const TEST_TONE_AMPLITUDE: f32 = 0.25;

/// The test tone: a 440 Hz sine of amplitude 0,25 for 48 000 frames, with a linear fade of
/// `MICROFADE_FRAMES` frames at each end, the same in both channels (06 §12).
#[derive(Debug, Default)]
pub(crate) struct TestTone {
    /// Frame of the tone that sounds next, or `None` when it is not sounding.
    next_frame: Option<u32>,
}

impl TestTone {
    /// Starts the tone. If it is already sounding, nothing changes (06 §12).
    pub(crate) fn start(&mut self) {
        if self.next_frame.is_none() {
            self.next_frame = Some(0);
        }
    }

    /// The sample of the next frame, the same for both channels: 0 when the tone is not
    /// sounding.
    pub(crate) fn next_sample(&mut self) -> f32 {
        let Some(frame) = self.next_frame else {
            return 0.0;
        };
        self.next_frame = Some(frame + 1).filter(|next| *next < TEST_TONE_FRAMES);
        test_tone_sample(frame)
    }
}

/// Sample `i` of the test tone, for `i < TEST_TONE_FRAMES`. The fade is that of the
/// microfades (06 §9.2): 0 on the first and last frames, and 1 from `MICROFADE_FRAMES` frames
/// away from both ends.
fn test_tone_sample(i: u32) -> f32 {
    // The phase, in cycles, is computed in integers so that it stays exact along the tone.
    let cycle =
        (u64::from(i) * TEST_TONE_HZ % u64::from(SAMPLE_RATE)) as f64 / f64::from(SAMPLE_RATE);
    let sine = (TAU * cycle).sin() as f32;
    let fade = MICROFADE_FRAMES as f32;
    let fade_in = i as f32 / fade;
    let fade_out = TEST_TONE_FRAMES.saturating_sub(1 + i) as f32 / fade;
    TEST_TONE_AMPLITUDE * sine * fade_in.min(fade_out).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tone_is_silent_until_it_starts_and_after_its_last_frame() {
        let mut tone = TestTone::default();
        assert_eq!(tone.next_sample(), 0.0);
        tone.start();
        let samples: Vec<f32> = (0..TEST_TONE_FRAMES + 10)
            .map(|_| tone.next_sample())
            .collect();
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[TEST_TONE_FRAMES as usize - 1], 0.0);
        assert!(samples[TEST_TONE_FRAMES as usize..]
            .iter()
            .all(|s| *s == 0.0));
        assert!(tone.next_frame.is_none());
    }

    #[test]
    fn the_fades_last_240_frames() {
        // (frame, gain of the fade)
        let cases = [
            (120, 0.5),
            (239, 239.0 / 240.0),
            (240, 1.0),
            (47_759, 1.0),
            (47_760, 239.0 / 240.0),
            (47_879, 0.5),
        ];
        for (i, gain) in cases {
            let sine = (TAU * f64::from(i) * 440.0 / 48_000.0).sin();
            let expected = 0.25 * gain * sine;
            let actual = f64::from(test_tone_sample(i));
            assert!(
                (actual - expected).abs() < 1e-7,
                "frame {i}: {actual} != {expected}"
            );
        }
    }

    #[test]
    fn starting_while_it_sounds_changes_nothing() {
        let mut tone = TestTone::default();
        tone.start();
        for _ in 0..1_000 {
            tone.next_sample();
        }
        tone.start();
        assert_eq!(tone.next_frame, Some(1_000));
    }
}
