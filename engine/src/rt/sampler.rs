//! The voices of the kit sounds of a slot (06 §8.2).
//!
//! A hit always starts on its exact frame: the sequencer computes the voices up to that frame,
//! fires the voice there and goes on (`sequencer`). So the choke groups and the voice limit act
//! on the frame of the hit, whatever the size of the block (RT-10), and a voice never waits
//! inside a subblock before it starts to sound.

use crate::constants::{MAX_VOICES, MICROFADE_FRAMES};
use crate::kits::Kits;

/// Voices of each slot: `MAX_VOICES` sounding plus 8 fading out (06 §19).
pub(crate) const VOICES_PER_SLOT: usize = MAX_VOICES + 8;
/// `MICROFADE_FRAMES` as the type of `Voice::fade_left`.
const FADE_FRAMES: u16 = MICROFADE_FRAMES as u16;

/// A kit sound that is sounding (06 §8.2).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Voice {
    active: bool,
    row: u8,
    /// 0 = no choke group.
    choke_group: u8,
    /// Index of the kit with which it was fired.
    kit: u8,
    /// Index of the sound within that kit (the row).
    sound: u8,
    /// Next frame of the sound to read.
    pos: usize,
    gain: f32,
    /// 0 = not fading out; otherwise, frames left of its fade.
    fade_left: u16,
    /// Firing order within the slot.
    age: u64,
}

impl Voice {
    fn fading(&self) -> bool {
        self.active && self.fade_left > 0
    }

    fn sounding(&self) -> bool {
        self.active && !self.fading()
    }
}

/// The fixed set of voices of a slot (06 §8.2).
#[derive(Debug, Clone)]
pub(crate) struct Voices {
    voices: [Voice; VOICES_PER_SLOT],
    /// Age of the next voice fired.
    next_age: u64,
}

impl Default for Voices {
    fn default() -> Self {
        Self {
            voices: [Voice::default(); VOICES_PER_SLOT],
            next_age: 0,
        }
    }
}

impl Voices {
    /// Fires the sound of `row` of kit `kit` with gain `gain` (06 §8.2):
    ///
    /// 1. if the sound has a choke group, every voice of that group and of another row that is
    ///    not fading out yet starts to fade out (RF-213);
    /// 2. if `MAX_VOICES` voices are sounding without fading out, the oldest starts to fade out;
    /// 3. the new voice takes the free voice with the lowest index or, if there is none, the
    ///    one that is fading out with the fewest frames left (the lowest index on a tie).
    pub(crate) fn fire(&mut self, kits: &Kits, kit: usize, row: usize, gain: f32) {
        let Some(sound) = kits.kits.get(kit).and_then(|k| k.sounds.get(row)) else {
            return;
        };
        let (Ok(kit), Ok(row)) = (u8::try_from(kit), u8::try_from(row)) else {
            return;
        };

        if sound.choke_group > 0 {
            for voice in &mut self.voices {
                if voice.sounding() && voice.choke_group == sound.choke_group && voice.row != row {
                    voice.fade_left = FADE_FRAMES;
                }
            }
        }

        let sounding = self.voices.iter().filter(|v| v.sounding()).count();
        if sounding >= MAX_VOICES {
            let oldest = self
                .voices
                .iter_mut()
                .filter(|v| v.sounding())
                .min_by_key(|v| v.age);
            if let Some(oldest) = oldest {
                oldest.fade_left = FADE_FRAMES;
            }
        }

        let free = self.voices.iter().position(|v| !v.active);
        // `min_by_key` keeps the first of the equal ones: the lowest index.
        let reused = || {
            let fading = self.voices.iter().enumerate().filter(|(_, v)| v.fading());
            fading
                .min_by_key(|(_, v)| v.fade_left)
                .map(|(index, _)| index)
        };
        let Some(index) = free.or_else(reused) else {
            return;
        };
        if let Some(voice) = self.voices.get_mut(index) {
            *voice = Voice {
                active: true,
                row,
                choke_group: sound.choke_group,
                kit,
                sound: row,
                pos: 0,
                gain,
                fade_left: 0,
                age: self.next_age,
            };
            self.next_age += 1;
        }
    }

    /// Adds the voices to `out`, interleaved stereo, and advances them (06 §8.2). The sounds
    /// are mono: the same sample goes to both channels.
    pub(crate) fn render(&mut self, kits: &Kits, out: &mut [f32]) {
        let (frames, _) = out.as_chunks_mut::<2>();
        for voice in self.voices.iter_mut().filter(|v| v.active) {
            let sound = kits
                .kits
                .get(usize::from(voice.kit))
                .and_then(|kit| kit.sounds.get(usize::from(voice.sound)));
            let Some(samples) = sound.map(|sound| sound.samples.as_slice()) else {
                voice.active = false;
                continue;
            };
            for frame in frames.iter_mut() {
                let Some(&x) = samples.get(voice.pos) else {
                    voice.active = false;
                    break;
                };
                let factor = if voice.fading() {
                    f32::from(voice.fade_left) / f32::from(FADE_FRAMES)
                } else {
                    1.0
                };
                let sample = x * voice.gain * factor;
                frame[0] += sample;
                frame[1] += sample;
                voice.pos += 1;
                if voice.fading() {
                    voice.fade_left -= 1;
                    if voice.fade_left == 0 {
                        voice.active = false;
                        break;
                    }
                }
            }
            if voice.pos >= samples.len() {
                voice.active = false;
            }
        }
    }

    /// Switches every voice off at once (06 §6.1, on reaching `Stopped`).
    pub(crate) fn silence(&mut self) {
        self.voices = [Voice::default(); VOICES_PER_SLOT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::KIT_ROWS;
    use crate::kits::{Kit, KitSound};

    const HAT_CLOSED: usize = 3;
    const HAT_OPEN: usize = 4;

    /// Three equal kits whose sounds last `frames` frames of value 1,0, without choke groups.
    fn kits(frames: usize) -> Kits {
        kits_with_hat_group(frames, 0)
    }

    /// Like [`kits`], with both hi-hats in choke group `group`.
    fn kits_with_hat_group(frames: usize, group: u8) -> Kits {
        let kit = Kit {
            sounds: std::array::from_fn(|row| KitSound {
                samples: vec![1.0; frames],
                choke_group: if row == HAT_CLOSED || row == HAT_OPEN {
                    group
                } else {
                    0
                },
            }),
        };
        Kits {
            kits: [kit.clone(), kit.clone(), kit],
        }
    }

    fn sounding(voices: &Voices) -> Vec<&Voice> {
        voices.voices.iter().filter(|v| v.sounding()).collect()
    }

    fn fading(voices: &Voices) -> Vec<&Voice> {
        voices.voices.iter().filter(|v| v.fading()).collect()
    }

    #[test]
    fn never_more_than_max_voices_sound_and_the_oldest_one_fades_out() {
        let kits = kits(96_000);
        let mut voices = Voices::default();
        // 8 rows during 6 steps, without computing anything between them: 48 hits.
        for hit in 0..6 * KIT_ROWS {
            let ages_before: Vec<u64> = sounding(&voices).iter().map(|v| v.age).collect();
            voices.fire(&kits, 0, hit % KIT_ROWS, 0.5);
            let sounding = sounding(&voices);
            assert!(sounding.len() <= MAX_VOICES, "hit {hit}");
            if let (MAX_VOICES, Some(&oldest)) = (ages_before.len(), ages_before.iter().min()) {
                assert!(sounding.iter().all(|v| v.age != oldest), "hit {hit}");
                assert_eq!(sounding.len(), MAX_VOICES, "hit {hit}");
            }
        }
        // The last voices are the newest: hits 16 to 47. The rest have been reused or fade out.
        let mut ages: Vec<u64> = sounding(&voices).iter().map(|v| v.age).collect();
        ages.sort_unstable();
        assert_eq!(ages, (16..48).collect::<Vec<u64>>());
    }

    #[test]
    fn without_a_free_voice_the_one_with_the_fewest_fade_frames_left_is_reused() {
        let kits = kits(96_000);
        let mut voices = Voices::default();
        for hit in 0..VOICES_PER_SLOT {
            voices.fire(&kits, 0, hit % 3, 0.5);
        }
        // 32 sounding and 8 fading out, all with 240 frames left; voice 2 has fewer.
        assert_eq!(fading(&voices).len(), 8);
        voices.voices[2].fade_left = 10;
        voices.voices[5].fade_left = 10;
        voices.fire(&kits, 0, 0, 0.25);
        assert_eq!(voices.voices[2].age, VOICES_PER_SLOT as u64);
        assert_eq!(voices.voices[2].gain, 0.25);
        assert!(voices.voices[2].sounding());
        assert_eq!(voices.voices[5].fade_left, 10);
    }

    #[test]
    fn a_choke_group_fades_out_the_other_rows_of_the_group_but_not_the_same_row() {
        let kits = kits_with_hat_group(96_000, 1);
        let mut voices = Voices::default();
        voices.fire(&kits, 0, HAT_OPEN, 1.0);
        voices.fire(&kits, 0, HAT_OPEN, 1.0);
        voices.fire(&kits, 0, 0, 1.0);
        assert_eq!(sounding(&voices).len(), 3);

        voices.fire(&kits, 0, HAT_CLOSED, 1.0);
        let faded: Vec<u8> = fading(&voices).iter().map(|v| v.row).collect();
        assert_eq!(faded, [HAT_OPEN as u8, HAT_OPEN as u8]);
        let rows: Vec<u8> = sounding(&voices).iter().map(|v| v.row).collect();
        assert_eq!(rows, [0, HAT_CLOSED as u8]);
    }

    #[test]
    fn a_voice_reads_its_sound_with_its_gain_and_ends_with_it() {
        let mut kits = kits(3);
        kits.kits[2].sounds[1].samples = vec![1.0, -0.5, 0.25];
        let mut voices = Voices::default();
        voices.fire(&kits, 2, 1, 0.5);
        let mut out = [0.0; 8];
        voices.render(&kits, &mut out[..4]);
        voices.render(&kits, &mut out[4..]);
        assert_eq!(out, [0.5, 0.5, -0.25, -0.25, 0.125, 0.125, 0.0, 0.0]);
        assert!(voices.voices.iter().all(|v| !v.active));
    }

    #[test]
    fn a_fading_voice_falls_to_0_in_240_frames() {
        let kits = kits(96_000);
        let mut voices = Voices::default();
        voices.fire(&kits, 0, HAT_OPEN, 1.0);
        voices.voices[0].fade_left = FADE_FRAMES;
        let mut out = vec![0.0; 2 * 300];
        voices.render(&kits, &mut out);
        for i in 0..240 {
            assert_eq!(out[2 * i], (240 - i) as f32 / 240.0, "frame {i}");
        }
        assert!(out[480..].iter().all(|s| *s == 0.0));
        assert!(!voices.voices[0].active);
    }

    #[test]
    fn silence_switches_every_voice_off() {
        let kits = kits(96_000);
        let mut voices = Voices::default();
        for row in 0..KIT_ROWS {
            voices.fire(&kits, 1, row, 1.0);
        }
        voices.silence();
        let mut out = [0.0; 20];
        voices.render(&kits, &mut out);
        assert_eq!(out, [0.0; 20]);
    }

    #[test]
    fn a_sound_outside_the_kits_does_not_fire() {
        let kits = kits(10);
        let mut voices = Voices::default();
        voices.fire(&kits, 3, 0, 1.0);
        voices.fire(&kits, 0, KIT_ROWS, 1.0);
        assert!(voices.voices.iter().all(|v| !v.active));
    }
}
