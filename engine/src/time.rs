//! Unit conversions of 05 §2. `src/core/time.ts` implements the same formulas; both are
//! checked against `fixtures/formulas/time.json` (08 §7). Every conversion is computed from
//! the start of the song, never by accumulating increments (05 §2.2).

use crate::constants::{
    MAX_SONG_FRAMES, PPQ, SAMPLE_RATE, STEPS_PER_BAR, TICKS_PER_STEP, TIME_SIGNATURES,
};

/// Frames per tick at one quarter note per minute: 48000 · 60 / 960 = 3000.
const TICK_FRAMES_AT_1_BPM: u64 = SAMPLE_RATE as u64 * 60 / PPQ;
/// Frames per step at one quarter note per minute: 3000 · 240 = 720000.
const STEP_FRAMES_AT_1_BPM: u64 = TICK_FRAMES_AT_1_BPM * TICKS_PER_STEP;

/// Time signature of the project (05 §2.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSignature {
    /// 4/4.
    FourFour = 0,
    /// 3/4.
    ThreeFour = 1,
    /// 6/8.
    SixEight = 2,
}

impl TimeSignature {
    /// Every time signature, in the order of [`TIME_SIGNATURES`].
    pub const ALL: [TimeSignature; 3] = [Self::FourFour, Self::ThreeFour, Self::SixEight];

    /// Name used in the project and the snapshot: `"4/4"`, `"3/4"` or `"6/8"`.
    pub fn name(self) -> &'static str {
        TIME_SIGNATURES[self as usize]
    }

    /// The time signature with this name, if it is one of [`TIME_SIGNATURES`].
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|signature| signature.name() == name)
    }

    /// Steps per bar.
    pub fn steps_per_bar(self) -> u64 {
        STEPS_PER_BAR[self as usize].1
    }

    /// Ticks per bar.
    pub fn ticks_per_bar(self) -> u64 {
        self.steps_per_bar() * TICKS_PER_STEP
    }

    /// Ticks per beat. The beat is a quarter note in 4/4 and 3/4, and an eighth note in
    /// 6/8 (glossary, 00 §7).
    pub fn ticks_per_beat(self) -> u64 {
        match self {
            Self::FourFour | Self::ThreeFour => PPQ,
            Self::SixEight => PPQ / 2,
        }
    }

    /// Beats per bar.
    pub fn beats_per_bar(self) -> u64 {
        self.ticks_per_bar() / self.ticks_per_beat()
    }
}

/// Frames per tick; not an integer in general.
pub fn frames_per_tick(tempo_bpm: u32) -> f64 {
    TICK_FRAMES_AT_1_BPM as f64 / f64::from(tempo_bpm)
}

/// Rounds `product / divisor` to the nearest integer. As 06 §8.1 requires, the product is
/// computed first, in `f64`, then the division, then the rounding; `f64::round` sends halves
/// up for the non-negative operands of 05 §2.2.
fn round_quotient(product: f64, divisor: f64) -> u64 {
    (product / divisor).round() as u64
}

/// Frame of a tick, counted from the start of the song.
pub fn tick_to_frame(tick: u64, tempo_bpm: u32) -> u64 {
    round_quotient(
        tick as f64 * TICK_FRAMES_AT_1_BPM as f64,
        f64::from(tempo_bpm),
    )
}

/// Tick of a frame, counted from the start of the song.
pub fn frame_to_tick(frame: u64, tempo_bpm: u32) -> u64 {
    round_quotient(
        frame as f64 * f64::from(tempo_bpm),
        TICK_FRAMES_AT_1_BPM as f64,
    )
}

/// Frame of step `step`, counted from the start of the song or of the pattern.
pub fn step_to_frame(step: u64, tempo_bpm: u32) -> u64 {
    round_quotient(
        step as f64 * STEP_FRAMES_AT_1_BPM as f64,
        f64::from(tempo_bpm),
    )
}

/// First tick of a bar.
pub fn bar_to_tick(bar: u64, time_signature: TimeSignature) -> u64 {
    bar * time_signature.ticks_per_bar()
}

/// First step of a bar.
pub fn bar_to_step(bar: u64, time_signature: TimeSignature) -> u64 {
    bar * time_signature.steps_per_bar()
}

/// Last tick that fits in the song at this tempo.
pub fn max_song_tick(tempo_bpm: u32) -> u64 {
    MAX_SONG_FRAMES * u64::from(tempo_bpm) / TICK_FRAMES_AT_1_BPM
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn number(value: &Value) -> u64 {
        value
            .as_u64()
            .unwrap_or_else(|| panic!("{value} is not a number"))
    }

    fn tempo(value: &Value) -> u32 {
        u32::try_from(number(value)).unwrap_or_else(|_| panic!("{value} is not a tempo"))
    }

    fn signature(value: &Value) -> TimeSignature {
        value
            .as_str()
            .and_then(TimeSignature::from_name)
            .unwrap_or_else(|| panic!("{value} is not a time signature"))
    }

    fn evaluate(function: &str, args: &[Value]) -> u64 {
        match function {
            "tickToFrame" => tick_to_frame(number(&args[0]), tempo(&args[1])),
            "frameToTick" => frame_to_tick(number(&args[0]), tempo(&args[1])),
            "stepToFrame" => step_to_frame(number(&args[0]), tempo(&args[1])),
            "barToTick" => bar_to_tick(number(&args[0]), signature(&args[1])),
            "barToStep" => bar_to_step(number(&args[0]), signature(&args[1])),
            "maxSongTick" => max_song_tick(tempo(&args[0])),
            other => panic!("unknown function {other} in fixtures/formulas/time.json"),
        }
    }

    #[test]
    fn formulas_match_the_fixture() {
        let text = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/formulas/time.json"
        ));
        // The fixture is part of the repository; a parse error is a broken fixture.
        let cases: Vec<Value> =
            serde_json::from_str(text).expect("fixtures/formulas/time.json is valid JSON");
        assert!(!cases.is_empty());

        let mut mismatches = Vec::new();
        for case in &cases {
            let function = case["fn"].as_str().expect("every case has a function name");
            let args = case["args"].as_array().expect("every case has arguments");
            let actual = evaluate(function, args);
            if actual != number(&case["result"]) {
                mismatches.push(format!("{case} gave {actual}"));
            }
        }
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }

    #[test]
    fn step_to_frame_at_97_bpm_gives_7423_and_14845() {
        assert_eq!(step_to_frame(1, 97), 7423);
        assert_eq!(step_to_frame(2, 97), 14845);
    }

    #[test]
    fn rounds_halves_up() {
        // 1 tick at 240 bpm is 12.5 frames; 75 frames at 60 bpm are 1.5 ticks.
        assert_eq!(tick_to_frame(1, 240), 13);
        assert_eq!(frame_to_tick(75, 60), 2);
    }

    #[test]
    fn frames_per_tick_is_3000_over_the_tempo() {
        assert_eq!(frames_per_tick(120), 25.0);
        assert_eq!(frames_per_tick(100), 30.0);
    }

    #[test]
    fn time_signatures_have_the_units_of_05_2_1() {
        let table = [
            (TimeSignature::FourFour, "4/4", 3840, 16, 4, 960),
            (TimeSignature::ThreeFour, "3/4", 2880, 12, 3, 960),
            (TimeSignature::SixEight, "6/8", 2880, 12, 6, 480),
        ];
        for (signature, name, ticks, steps, beats, beat_ticks) in table {
            assert_eq!(signature.name(), name);
            assert_eq!(TimeSignature::from_name(name), Some(signature));
            assert_eq!(signature.ticks_per_bar(), ticks, "{name}");
            assert_eq!(signature.steps_per_bar(), steps, "{name}");
            assert_eq!(signature.beats_per_bar(), beats, "{name}");
            assert_eq!(signature.ticks_per_beat(), beat_ticks, "{name}");
        }
        assert_eq!(TimeSignature::from_name("5/4"), None);
    }
}
