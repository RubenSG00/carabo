//! From steps to hits (06 §8.1).

use crate::constants::{LEVEL_GAINS, PPQ, SAMPLE_RATE, TEMPO_MAX, TICKS_PER_STEP};
use crate::kits::Kits;
use crate::project::{Pattern, RhythmTrack};
use crate::rt::sampler::Voices;
use crate::rt::MAX_BLOCK;
use crate::time::step_to_frame;

/// Frames per step at one quarter note per minute: 48 000 · 60 / 960 · 240 = 720 000.
const STEP_FRAMES_AT_1_BPM: u64 = SAMPLE_RATE as u64 * 60 / PPQ * TICKS_PER_STEP;
/// Most steps that the loop of [`play_pattern`] visits in one subblock (RT-07): those that fit
/// in `MAX_BLOCK` frames at `TEMPO_MAX`, plus the one before the subblock, the two that rounding
/// can add at its ends and the one that ends the loop.
const MAX_STEPS_PER_SUBBLOCK: u64 = MAX_BLOCK as u64 * TEMPO_MAX as u64 / STEP_FRAMES_AT_1_BPM + 5;

/// The step that contains `frame`: `floor(frame · tempoBpm / 720 000)`, in exact integers
/// (06 §8.1, §18.1).
pub(crate) fn step_at(frame: u64, tempo_bpm: u32) -> u64 {
    frame.saturating_mul(u64::from(tempo_bpm)) / STEP_FRAMES_AT_1_BPM
}

/// Plays `pattern` of `track` in pattern mode (`q = s`) over a subblock that starts at position
/// `position`, and adds the voices of the track's slot to `out`, interleaved stereo (06 §8.1).
///
/// Each hit is fired on its exact frame: the voices are computed up to the frame of each step
/// that falls in the subblock, and its hits are fired there.
pub(crate) fn play_pattern(
    track: &RhythmTrack,
    pattern: &Pattern,
    tempo_bpm: u32,
    position: u64,
    voices: &mut Voices,
    kits: &Kits,
    out: &mut [f32],
) {
    let frames = (out.len() / 2) as u64;
    let end = position + frames;
    let first = step_at(position, tempo_bpm).saturating_sub(1);
    let mut rendered = 0;
    for step in first..first + MAX_STEPS_PER_SUBBLOCK {
        let frame = step_to_frame(step, tempo_bpm);
        if frame >= end {
            break;
        }
        if frame < position {
            continue;
        }
        // `frame` lies in [position, end), so `at` is within the subblock.
        let at = (frame - position) as usize;
        if let Some(part) = out.get_mut(2 * rendered..2 * at) {
            voices.render(kits, part);
        }
        rendered = at;
        let Some(levels) = usize::try_from(step)
            .ok()
            .and_then(|q| pattern.steps.get(q))
        else {
            continue;
        };
        for (row, (&level, settings)) in levels.iter().zip(&track.rows).enumerate() {
            let Some(level_gain) = LEVEL_GAINS.get(usize::from(level)) else {
                continue;
            };
            if level > 0 && !settings.mute {
                voices.fire(kits, track.kit, row, level_gain * settings.gain);
            }
        }
    }
    if let Some(part) = out.get_mut(2 * rendered..) {
        voices.render(kits, part);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_frames_at_1_bpm_is_720000() {
        assert_eq!(STEP_FRAMES_AT_1_BPM, 720_000);
    }

    #[test]
    fn step_at_is_the_floor_of_the_exact_quotient() {
        assert_eq!(step_at(0, 120), 0);
        assert_eq!(step_at(5_999, 120), 0);
        assert_eq!(step_at(6_000, 120), 1);
        // At 97, step 1 starts on frame round(720 000 / 97) = 7 423, and 7 422 · 97 < 720 000.
        assert_eq!(step_at(7_422, 97), 0);
        assert_eq!(step_at(7_423, 97), 1);
        assert_eq!(step_at(u64::MAX, 240), u64::MAX / 720_000);
    }

    #[test]
    fn the_loop_bound_covers_every_step_of_a_subblock() {
        // At the fastest tempo, a subblock of MAX_BLOCK frames holds at most 2 steps.
        for tempo in [TEMPO_MAX, 239, 97, 40] {
            for position in (0..200_000).step_by(997) {
                let first = step_at(position, tempo).saturating_sub(1);
                let end = position + MAX_BLOCK as u64;
                let visited = (first..)
                    .take_while(|&s| step_to_frame(s, tempo) < end)
                    .count() as u64;
                assert!(visited < MAX_STEPS_PER_SUBBLOCK, "{tempo}, {position}");
            }
        }
    }
}
