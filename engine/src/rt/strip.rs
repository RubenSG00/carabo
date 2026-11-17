//! The channel strip of a track (06 §10): for now, volume and pan.
// TODO(T-040): the ramps of 480 frames and the meter (06 §10.2); TODO(T-041): the equalizer;
// TODO(T-042): the reverb.

use crate::project::Strip;

/// Applies volume and pan to the source of a track and adds it to the master bus, both
/// interleaved stereo of the same length (06 §10.1, §10.2):
///
/// ```text
/// targetLeft  = volume · audible · panLeft
/// targetRight = volume · audible · panRight
/// ```
pub(crate) fn mix(strip: &Strip, audible: f32, source: &[f32], bus: &mut [f32]) {
    let left = strip.volume * audible * strip.pan_left;
    let right = strip.volume * audible * strip.pan_right;
    let (source, _) = source.as_chunks::<2>();
    let (bus, _) = bus.as_chunks_mut::<2>();
    for (sum, frame) in bus.iter_mut().zip(source) {
        sum[0] += frame[0] * left;
        sum[1] += frame[1] * right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRIP: Strip = Strip {
        volume: 0.5,
        pan_left: 1.0,
        pan_right: 0.25,
    };

    #[test]
    fn volume_audible_and_pan_multiply_each_channel_and_the_result_is_added() {
        let source = [1.0, 1.0, -0.5, 0.5];
        let mut bus = [0.125, 0.0, 0.0, 1.0];
        mix(&STRIP, 1.0, &source, &mut bus);
        assert_eq!(bus, [0.625, 0.125, -0.25, 1.0625]);
    }

    #[test]
    fn a_track_that_is_not_audible_adds_nothing() {
        let mut bus = [0.25; 4];
        mix(&STRIP, 0.0, &[1.0; 4], &mut bus);
        assert_eq!(bus, [0.25; 4]);
    }
}
