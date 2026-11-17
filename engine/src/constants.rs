//! The global constants of 00 §4 that the engine uses. This is the only Rust file that holds
//! these values (07 §7); `src/core/constants.ts` holds the same names on the TypeScript side,
//! and both are checked against `fixtures/constants.json` (08 §7).

/// Internal sample rate, in Hz.
pub const SAMPLE_RATE: u32 = 48_000;
/// Ticks per quarter note.
pub const PPQ: u64 = 960;
/// Ticks per step (a sixteenth note).
pub const TICKS_PER_STEP: u64 = 240;

/// Lowest tempo, in quarter notes per minute.
pub const TEMPO_MIN: u32 = 40;
/// Highest tempo, in quarter notes per minute.
pub const TEMPO_MAX: u32 = 240;

/// Allowed time signatures, as written in the project and the snapshot.
pub const TIME_SIGNATURES: [&str; 3] = ["4/4", "3/4", "6/8"];
/// Steps per bar of each time signature, in the order of [`TIME_SIGNATURES`].
pub const STEPS_PER_BAR: [(&str, u64); 3] = [("4/4", 16), ("3/4", 12), ("6/8", 12)];

/// Maximum number of tracks, and number of slots of the engine.
pub const MAX_TRACKS: usize = 16;
/// Maximum song length: 10 minutes.
pub const MAX_SONG_FRAMES: u64 = 28_800_000;
/// Maximum number of patterns per rhythm track.
pub const MAX_PATTERNS_PER_TRACK: usize = 32;
/// Allowed pattern lengths, in bars.
pub const PATTERN_BARS: [u64; 3] = [1, 2, 4];
/// Rows (sounds) per kit.
pub const KIT_ROWS: usize = 8;
/// Micro-fade length: 5 ms.
pub const MICROFADE_FRAMES: u64 = 240;
/// Count-in length, in bars.
pub const COUNT_IN_BARS: u64 = 1;

/// Lowest value of a volume fader, in dB; below it the gain is 0.
pub const FADER_MIN_DB: f64 = -60.0;
/// Highest value of a volume fader, in dB.
pub const FADER_MAX_DB: f64 = 6.0;
/// Lowest row volume, in dB.
pub const ROW_GAIN_MIN_DB: f64 = -48.0;
/// Highest row volume, in dB.
pub const ROW_GAIN_MAX_DB: f64 = 6.0;
/// Lowest gain of an equalizer band, in dB.
pub const EQ_GAIN_MIN_DB: f64 = -12.0;
/// Highest gain of an equalizer band, in dB.
pub const EQ_GAIN_MAX_DB: f64 = 12.0;
/// Frequency of the low equalizer band, in Hz.
pub const EQ_LOW_HZ: f64 = 150.0;
/// Frequency of the mid equalizer band, in Hz.
pub const EQ_MID_HZ: f64 = 1000.0;
/// Frequency of the high equalizer band, in Hz.
pub const EQ_HIGH_HZ: f64 = 6000.0;

/// Gain of a hit by level: 0 off, 1 soft, 2 normal, 3 loud.
pub const LEVEL_GAINS: [f32; 4] = [0.0, 0.45, 0.75, 1.0];
/// Simultaneous voices per rhythm track.
pub const MAX_VOICES: usize = 32;
/// Tail added when exporting: 2 seconds.
pub const EXPORT_TAIL_FRAMES: u64 = 96_000;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn fixture() -> Value {
        let text = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/constants.json"
        ));
        // The fixture is part of the repository; a parse error is a broken fixture.
        serde_json::from_str(text).expect("fixtures/constants.json is valid JSON")
    }

    fn fixture_number(fixture: &Value, name: &str) -> f64 {
        fixture[name]
            .as_f64()
            .unwrap_or_else(|| panic!("{name} is missing from fixtures/constants.json"))
    }

    #[test]
    fn numeric_constants_match_the_fixture() {
        let fixture = fixture();
        let constants: [(&str, f64); 22] = [
            ("SAMPLE_RATE", f64::from(SAMPLE_RATE)),
            ("PPQ", PPQ as f64),
            ("TICKS_PER_STEP", TICKS_PER_STEP as f64),
            ("TEMPO_MIN", f64::from(TEMPO_MIN)),
            ("TEMPO_MAX", f64::from(TEMPO_MAX)),
            ("MAX_TRACKS", MAX_TRACKS as f64),
            ("MAX_SONG_FRAMES", MAX_SONG_FRAMES as f64),
            ("MAX_PATTERNS_PER_TRACK", MAX_PATTERNS_PER_TRACK as f64),
            ("KIT_ROWS", KIT_ROWS as f64),
            ("MICROFADE_FRAMES", MICROFADE_FRAMES as f64),
            ("COUNT_IN_BARS", COUNT_IN_BARS as f64),
            ("FADER_MIN_DB", FADER_MIN_DB),
            ("FADER_MAX_DB", FADER_MAX_DB),
            ("ROW_GAIN_MIN_DB", ROW_GAIN_MIN_DB),
            ("ROW_GAIN_MAX_DB", ROW_GAIN_MAX_DB),
            ("EQ_GAIN_MIN_DB", EQ_GAIN_MIN_DB),
            ("EQ_GAIN_MAX_DB", EQ_GAIN_MAX_DB),
            ("EQ_LOW_HZ", EQ_LOW_HZ),
            ("EQ_MID_HZ", EQ_MID_HZ),
            ("EQ_HIGH_HZ", EQ_HIGH_HZ),
            ("MAX_VOICES", MAX_VOICES as f64),
            ("EXPORT_TAIL_FRAMES", EXPORT_TAIL_FRAMES as f64),
        ];
        for (name, value) in constants {
            assert_eq!(fixture_number(&fixture, name), value, "{name}");
        }
    }

    #[test]
    fn time_signatures_match_the_fixture() {
        let fixture = fixture();
        assert_eq!(fixture["TIME_SIGNATURES"], json!(TIME_SIGNATURES));

        let steps = fixture["STEPS_PER_BAR"]
            .as_object()
            .expect("STEPS_PER_BAR is an object in fixtures/constants.json");
        assert_eq!(steps.len(), STEPS_PER_BAR.len());
        for (signature, steps_per_bar) in STEPS_PER_BAR {
            assert_eq!(
                steps[signature].as_u64(),
                Some(steps_per_bar),
                "{signature}"
            );
        }
    }

    #[test]
    fn pattern_bars_match_the_fixture() {
        assert_eq!(fixture()["PATTERN_BARS"], json!(PATTERN_BARS));
    }

    #[test]
    fn steps_per_bar_follows_the_order_of_time_signatures() {
        for (index, (signature, _)) in STEPS_PER_BAR.iter().enumerate() {
            assert_eq!(*signature, TIME_SIGNATURES[index]);
        }
    }

    #[test]
    fn level_gains_match_the_fixture() {
        let gains: Vec<f32> = fixture()["LEVEL_GAINS"]
            .as_array()
            .expect("LEVEL_GAINS is an array in fixtures/constants.json")
            .iter()
            .map(|gain| gain.as_f64().expect("every level gain is a number") as f32)
            .collect();
        assert_eq!(gains, LEVEL_GAINS);
    }
}
