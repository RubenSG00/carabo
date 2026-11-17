//! Contract «fórmulas en todo el rango» (08 §7): for every tempo and every step from 0 to
//! 5 000, `step_to_frame`, which works in `f64` (06 §8.1), gives the same frame as the
//! formula of 05 §2.2 computed in integers.

use carabo_engine::constants::{TEMPO_MAX, TEMPO_MIN};
use carabo_engine::time::step_to_frame;

/// `round(step · 720000 / tempo)` in integers, with halves going up (05 §2.2).
fn integer_step_frame(step: u64, tempo: u64) -> u64 {
    (2 * step * 720_000 + tempo) / (2 * tempo)
}

#[test]
fn step_to_frame_matches_integer_rounding_for_every_tempo_and_step() {
    for tempo in TEMPO_MIN..=TEMPO_MAX {
        for step in 0..=5_000 {
            assert_eq!(
                step_to_frame(step, tempo),
                integer_step_frame(step, u64::from(tempo)),
                "tempo {tempo}, step {step}"
            );
        }
    }
}
