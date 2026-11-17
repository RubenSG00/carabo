//! `EngineProject`: the snapshot prepared for the audio thread (06 §5.3), and the `Preparer`
//! with which the control thread builds it and keeps the slot of each track (06 §5.4).

use crate::constants::{FADER_MIN_DB, KIT_ROWS, MAX_TRACKS};
use crate::kits::kit_index;
use crate::snapshot::{
    ChannelStrip, EngineSnapshot, RowSettings, SnapshotPattern, SnapshotRhythmTrack, SnapshotTrack,
};

/// A snapshot prepared for the output thread (06 §5.3): immutable, with every value already
/// converted, so that the output thread neither searches nor computes anything costly.
// TODO(T-033): the generation of each slot and `pattern_target` (06 §5.3, §5.4).
// TODO(T-040): the master volume, the equalizer coefficients and the reverb parameters.
// TODO(T-043): the segments of the rhythm tracks, mute, solo, `any_solo` and `song_end_frame`.
// TODO(T-044): the audio tracks and `audio_changed`.
// TODO(T-047): steps, beats and ticks per beat of the time signature.
#[derive(Debug, Default)]
pub struct EngineProject {
    /// Quarter notes per minute, between `TEMPO_MIN` and `TEMPO_MAX`.
    pub(crate) tempo_bpm: u32,
    /// The rhythm tracks, in the order of the snapshot.
    pub(crate) rhythm_tracks: Vec<RhythmTrack>,
}

impl EngineProject {
    /// The rhythm track that uses `slot`, if any.
    pub(crate) fn rhythm_track(&self, slot: usize) -> Option<&RhythmTrack> {
        self.rhythm_tracks.iter().find(|track| track.slot == slot)
    }
}

/// A prepared rhythm track (06 §5.3).
#[derive(Debug)]
pub(crate) struct RhythmTrack {
    /// Its slot, from 0 to `MAX_TRACKS − 1` (06 §5.4).
    pub(crate) slot: usize,
    /// Index of its kit in `KIT_IDS`.
    pub(crate) kit: usize,
    pub(crate) rows: [Row; KIT_ROWS],
    pub(crate) patterns: Vec<Pattern>,
    pub(crate) strip: Strip,
}

/// Gain and mute of a row of a rhythm track.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Row {
    /// `10^(gainDb / 20)`.
    pub(crate) gain: f32,
    pub(crate) mute: bool,
}

/// A prepared pattern.
#[derive(Debug, PartialEq)]
pub(crate) struct Pattern {
    pub(crate) length_steps: u64,
    /// The level of each row on each step: `steps[q][r]` is the level of row `r` on step `q`.
    pub(crate) steps: Vec<[u8; KIT_ROWS]>,
}

/// The target gains of a channel strip, before `audible` (06 §10.2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Strip {
    /// 0 if `gainDb ≤ FADER_MIN_DB`; `10^(gainDb / 20)` otherwise.
    pub(crate) volume: f32,
    /// 1 if `pan ≤ 0`; `1 − pan/100` otherwise.
    pub(crate) pan_left: f32,
    /// 1 if `pan ≥ 0`; `1 + pan/100` otherwise.
    pub(crate) pan_right: f32,
}

/// What the control thread keeps between snapshots: the slot of each track (06 §5.4) and the
/// last snapshot received (06 §5.3), where `play` and `audition` look up tracks and patterns
/// (06 §5.1).
#[derive(Debug, Default)]
pub struct Preparer {
    /// The id of the track that uses each slot.
    slots: [Option<String>; MAX_TRACKS],
    last: Option<EngineSnapshot>,
}

impl Preparer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepares `snapshot`, which must be valid (06 §5.5), and keeps it as the last one. A
    /// track keeps its slot while it is in the successive snapshots; a new one takes the free
    /// slot with the lowest number (06 §5.4).
    pub fn prepare(&mut self, snapshot: EngineSnapshot) -> EngineProject {
        self.assign_slots(&snapshot);
        let tempo_bpm = snapshot.tempo_bpm as u32;
        let rhythm_tracks = snapshot
            .tracks
            .iter()
            .filter_map(|track| match track {
                SnapshotTrack::Rhythm(rhythm) => Some(self.prepare_rhythm(rhythm)),
                SnapshotTrack::Audio(_) => None,
            })
            .collect();
        self.last = Some(snapshot);
        EngineProject {
            tempo_bpm,
            rhythm_tracks,
        }
    }

    /// The slot of the rhythm track `track_id` of the last snapshot and the index of its
    /// pattern `pattern_id`, if both exist.
    pub fn pattern(&self, track_id: &str, pattern_id: &str) -> Option<(usize, usize)> {
        let track = self.last_rhythm_track(track_id)?;
        let index = track.patterns.iter().position(|p| p.id == pattern_id)?;
        Some((self.slot(track_id)?, index))
    }

    /// The slot of the rhythm track `track_id` of the last snapshot, if it exists.
    pub fn rhythm_slot(&self, track_id: &str) -> Option<usize> {
        self.last_rhythm_track(track_id)?;
        self.slot(track_id)
    }

    /// The id and the slot of each track of the last snapshot, in its order (06 §18.1).
    pub fn track_slots(&self) -> Vec<(String, usize)> {
        let Some(snapshot) = &self.last else {
            return Vec::new();
        };
        snapshot
            .tracks
            .iter()
            .filter_map(|track| Some((track.id().to_string(), self.slot(track.id())?)))
            .collect()
    }

    /// Frees the slots of the tracks that are no longer in `snapshot`, then gives each new
    /// track the free slot with the lowest number.
    // TODO(T-033): the generation of each slot (06 §5.4).
    fn assign_slots(&mut self, snapshot: &EngineSnapshot) {
        for slot in &mut self.slots {
            let gone = slot
                .as_deref()
                .is_some_and(|id| !snapshot.tracks.iter().any(|track| track.id() == id));
            if gone {
                *slot = None;
            }
        }
        for track in &snapshot.tracks {
            if self.slot(track.id()).is_some() {
                continue;
            }
            // A valid snapshot has at most MAX_TRACKS tracks, so there is always a free slot.
            if let Some(free) = self.slots.iter_mut().find(|slot| slot.is_none()) {
                *free = Some(track.id().to_string());
            }
        }
    }

    fn slot(&self, track_id: &str) -> Option<usize> {
        self.slots
            .iter()
            .position(|slot| slot.as_deref() == Some(track_id))
    }

    fn last_rhythm_track(&self, track_id: &str) -> Option<&SnapshotRhythmTrack> {
        self.last
            .as_ref()?
            .tracks
            .iter()
            .find_map(|track| match track {
                SnapshotTrack::Rhythm(rhythm) if rhythm.id == track_id => Some(rhythm),
                _ => None,
            })
    }

    fn prepare_rhythm(&self, track: &SnapshotRhythmTrack) -> RhythmTrack {
        let rows = std::array::from_fn(|r| prepare_row(track.rows.get(r)));
        RhythmTrack {
            slot: self.slot(&track.id).unwrap_or_default(),
            kit: kit_index(&track.kit_id).unwrap_or_default(),
            rows,
            patterns: track.patterns.iter().map(prepare_pattern).collect(),
            strip: prepare_strip(&track.strip),
        }
    }
}

/// `10^(db / 20)` (00 §3.5).
fn db_to_gain(db: f64) -> f32 {
    10f64.powf(db / 20.0) as f32
}

fn prepare_row(row: Option<&RowSettings>) -> Row {
    row.map_or(
        Row {
            gain: 0.0,
            mute: true,
        },
        |row| Row {
            gain: db_to_gain(row.gain_db),
            mute: row.mute,
        },
    )
}

fn prepare_pattern(pattern: &SnapshotPattern) -> Pattern {
    let length_steps = u64::try_from(pattern.length_steps).unwrap_or_default();
    let level = |row: usize, step: usize| {
        let level = pattern.steps.get(row).and_then(|levels| levels.get(step));
        level
            .and_then(|&level| u8::try_from(level).ok())
            .unwrap_or_default()
    };
    let steps = (0..pattern.steps.first().map_or(0, Vec::len))
        .map(|step| std::array::from_fn(|row| level(row, step)))
        .collect();
    Pattern {
        length_steps,
        steps,
    }
}

/// The target gains of a channel strip (06 §10.2).
fn prepare_strip(strip: &ChannelStrip) -> Strip {
    let volume = if strip.gain_db <= FADER_MIN_DB {
        0.0
    } else {
        db_to_gain(strip.gain_db)
    };
    let pan = strip.pan;
    Strip {
        volume,
        pan_left: if pan <= 0.0 {
            1.0
        } else {
            (1.0 - pan / 100.0) as f32
        },
        pan_right: if pan >= 0.0 {
            1.0
        } else {
            (1.0 + pan / 100.0) as f32
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::{EffectUnit, EqUnit, MasterStrip, ReverbUnit, SNAPSHOT_VERSION};

    fn strip(gain_db: f64, pan: f64) -> ChannelStrip {
        ChannelStrip {
            gain_db,
            pan,
            mute: false,
            solo: false,
            effects: vec![
                EffectUnit::Eq(EqUnit {
                    enabled: false,
                    low_db: 0.0,
                    mid_db: 0.0,
                    high_db: 0.0,
                }),
                EffectUnit::Reverb(ReverbUnit {
                    enabled: false,
                    amount: 20.0,
                    size: 50.0,
                }),
            ],
        }
    }

    fn rhythm(id: &str) -> SnapshotTrack {
        SnapshotTrack::Rhythm(SnapshotRhythmTrack {
            id: id.to_string(),
            strip: strip(0.0, 0.0),
            kit_id: "electronic".to_string(),
            rows: (0..KIT_ROWS)
                .map(|r| RowSettings {
                    gain_db: -(r as f64),
                    mute: r == 5,
                })
                .collect(),
            patterns: vec![
                SnapshotPattern {
                    id: "a".to_string(),
                    length_steps: 12,
                    steps: (0..KIT_ROWS)
                        .map(|r| (0..12).map(|s| i64::from(s == r) * 2).collect())
                        .collect(),
                },
                SnapshotPattern {
                    id: "b".to_string(),
                    length_steps: 24,
                    steps: vec![vec![3; 24]; KIT_ROWS],
                },
            ],
            segments: Vec::new(),
        })
    }

    fn snapshot(ids: &[&str]) -> EngineSnapshot {
        EngineSnapshot {
            snapshot_version: SNAPSHOT_VERSION,
            tempo_bpm: 97.0,
            time_signature: "3/4".to_string(),
            tracks: ids.iter().map(|id| rhythm(id)).collect(),
            master: MasterStrip { gain_db: 0.0 },
        }
    }

    fn slots(preparer: &Preparer) -> Vec<(String, usize)> {
        preparer.track_slots()
    }

    fn pairs(list: &[(&str, usize)]) -> Vec<(String, usize)> {
        list.iter().map(|(id, s)| (id.to_string(), *s)).collect()
    }

    #[test]
    fn a_track_keeps_its_slot_and_a_new_one_takes_the_lowest_free_slot() {
        let mut preparer = Preparer::new();
        preparer.prepare(snapshot(&["a", "b", "c"]));
        assert_eq!(slots(&preparer), pairs(&[("a", 0), ("b", 1), ("c", 2)]));

        // "b" goes away and "d" and "e" arrive: "d" takes slot 1, "e" slot 3.
        let project = preparer.prepare(snapshot(&["d", "c", "a", "e"]));
        assert_eq!(
            slots(&preparer),
            pairs(&[("d", 1), ("c", 2), ("a", 0), ("e", 3)])
        );
        let prepared: Vec<usize> = project.rhythm_tracks.iter().map(|t| t.slot).collect();
        assert_eq!(prepared, [1, 2, 0, 3]);
        assert_eq!(project.rhythm_track(0).map(|t| t.slot), Some(0));
        assert!(project.rhythm_track(4).is_none());
    }

    #[test]
    fn sixteen_tracks_take_the_sixteen_slots() {
        let ids: Vec<String> = (0..MAX_TRACKS).map(|i| format!("t{i}")).collect();
        let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
        let mut preparer = Preparer::new();
        preparer.prepare(snapshot(&ids));
        let used: Vec<usize> = slots(&preparer).iter().map(|(_, s)| *s).collect();
        assert_eq!(used, (0..MAX_TRACKS).collect::<Vec<_>>());
    }

    #[test]
    fn patterns_and_rhythm_tracks_are_found_in_the_last_snapshot() {
        let mut preparer = Preparer::new();
        assert_eq!(preparer.pattern("x", "a"), None);
        preparer.prepare(snapshot(&["x", "y"]));
        assert_eq!(preparer.pattern("y", "b"), Some((1, 1)));
        assert_eq!(preparer.pattern("x", "a"), Some((0, 0)));
        assert_eq!(preparer.pattern("x", "c"), None);
        assert_eq!(preparer.pattern("z", "a"), None);
        assert_eq!(preparer.rhythm_slot("y"), Some(1));
        assert_eq!(preparer.rhythm_slot("z"), None);
    }

    #[test]
    fn a_rhythm_track_is_prepared_with_its_kit_gains_and_levels() {
        let mut preparer = Preparer::new();
        let project = preparer.prepare(snapshot(&["x"]));
        assert_eq!(project.tempo_bpm, 97);
        let track = &project.rhythm_tracks[0];
        assert_eq!(track.kit, 1);
        assert_eq!(
            track.rows[0],
            Row {
                gain: 1.0,
                mute: false
            }
        );
        assert!((track.rows[6].gain - 0.501_187_2).abs() < 1e-6);
        assert!(track.rows[5].mute);

        let a = &track.patterns[0];
        assert_eq!(a.length_steps, 12);
        assert_eq!(a.steps.len(), 12);
        assert_eq!(a.steps[3], [0, 0, 0, 2, 0, 0, 0, 0]);
        assert_eq!(a.steps[10], [0; KIT_ROWS]);
        assert_eq!(track.patterns[1].steps, vec![[3; KIT_ROWS]; 24]);
    }

    #[test]
    fn strip_gains_follow_06_10_2() {
        let cases = [
            ((0.0, 0.0), (1.0, 1.0, 1.0)),
            ((-6.0, 0.0), (0.501_187_2, 1.0, 1.0)),
            ((FADER_MIN_DB, 0.0), (0.0, 1.0, 1.0)),
            ((6.0, 100.0), (1.995_262_3, 0.0, 1.0)),
            ((0.0, -50.0), (1.0, 1.0, 0.5)),
            ((0.0, 25.0), (1.0, 0.75, 1.0)),
        ];
        for ((gain_db, pan), (volume, left, right)) in cases {
            let s = prepare_strip(&strip(gain_db, pan));
            assert!(
                (s.volume - volume).abs() < 1e-6,
                "{gain_db} dB: {}",
                s.volume
            );
            assert_eq!((s.pan_left, s.pan_right), (left, right), "pan {pan}");
        }
    }

    #[test]
    fn audio_tracks_have_a_slot_but_are_not_prepared_yet() {
        let mut snap = snapshot(&["x"]);
        snap.tracks.insert(
            0,
            SnapshotTrack::Audio(crate::snapshot::SnapshotAudioTrack {
                id: "voice".to_string(),
                strip: strip(0.0, 0.0),
                segments: Vec::new(),
            }),
        );
        let mut preparer = Preparer::new();
        let project = preparer.prepare(snap);
        assert_eq!(slots(&preparer), pairs(&[("voice", 0), ("x", 1)]));
        assert_eq!(project.rhythm_tracks.len(), 1);
        assert_eq!(project.rhythm_tracks[0].slot, 1);
        assert_eq!(preparer.rhythm_slot("voice"), None);
    }
}
