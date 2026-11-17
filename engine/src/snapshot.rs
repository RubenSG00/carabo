//! The engine snapshot of 05 §10.1 and its validation (06 §5.5).
//!
//! The fields keep the JSON types that the interface sends: numbers that the validation bounds
//! are signed integers or `f64`, and the time signature and the kit are strings. That way any
//! value the interface can send is deserialized and reaches [`EngineSnapshot::validate`], which
//! rejects it with `INVALID_SNAPSHOT` (06 §5.1).

use serde::Deserialize;

use crate::constants::{
    EQ_GAIN_MAX_DB, EQ_GAIN_MIN_DB, FADER_MAX_DB, FADER_MIN_DB, KIT_ROWS, LEVEL_GAINS,
    MAX_PATTERNS_PER_TRACK, MAX_SONG_FRAMES, MAX_TRACKS, PATTERN_BARS, ROW_GAIN_MAX_DB,
    ROW_GAIN_MIN_DB, TEMPO_MAX, TEMPO_MIN,
};
use crate::error::EngineError;
use crate::kits::kit_index;
use crate::time::{step_to_frame, TimeSignature};

/// The only snapshot version of 05 §10.1.
pub const SNAPSHOT_VERSION: i64 = 1;

/// Pan range: from the left speaker (−100) to the right one (+100) (05 §3.1).
const PAN_RANGE: (f64, f64) = (-100.0, 100.0);
/// Range of the reverb amount and size (05 §3.1).
const REVERB_RANGE: (f64, f64) = (0.0, 100.0);
/// Highest level of a hit: the last index of `LEVEL_GAINS`.
const MAX_LEVEL: i64 = LEVEL_GAINS.len() as i64 - 1;

/// Everything that must sound (05 §10.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineSnapshot {
    pub snapshot_version: i64,
    /// Quarter notes per minute; an integer once validated.
    pub tempo_bpm: f64,
    /// `"4/4"`, `"3/4"` or `"6/8"` once validated.
    pub time_signature: String,
    /// In the order of the tracks.
    pub tracks: Vec<SnapshotTrack>,
    pub master: MasterStrip,
}

/// A track of the snapshot, tagged by its `type`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SnapshotTrack {
    Rhythm(SnapshotRhythmTrack),
    Audio(SnapshotAudioTrack),
}

impl SnapshotTrack {
    /// Identifier of the track.
    pub fn id(&self) -> &str {
        match self {
            Self::Rhythm(track) => &track.id,
            Self::Audio(track) => &track.id,
        }
    }

    /// Channel strip of the track.
    pub fn strip(&self) -> &ChannelStrip {
        match self {
            Self::Rhythm(track) => &track.strip,
            Self::Audio(track) => &track.strip,
        }
    }
}

/// A rhythm track (05 §10.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotRhythmTrack {
    pub id: String,
    pub strip: ChannelStrip,
    /// One of the kit identifiers once validated.
    pub kit_id: String,
    /// `KIT_ROWS` rows once validated.
    pub rows: Vec<RowSettings>,
    /// Every pattern of the track.
    pub patterns: Vec<SnapshotPattern>,
    /// Without overlaps, sorted by `start_step`.
    pub segments: Vec<RhythmSegment>,
}

/// Volume and mute of a row of a rhythm track (05 §3.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowSettings {
    pub gain_db: f64,
    pub mute: bool,
}

/// A pattern of a rhythm track (05 §10.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotPattern {
    pub id: String,
    /// Bars · steps per bar.
    pub length_steps: i64,
    /// `KIT_ROWS` rows of `length_steps` levels, 0..3.
    pub steps: Vec<Vec<i64>>,
}

/// A stretch of a rhythm lane where one pattern sounds (05 §10.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RhythmSegment {
    pub pattern_id: String,
    /// Song step where the segment starts.
    pub start_step: i64,
    /// At least 1.
    pub length_steps: i64,
    /// Steps elapsed since the start of its block.
    pub phase_steps: i64,
}

/// An audio track (05 §10.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotAudioTrack {
    pub id: String,
    pub strip: ChannelStrip,
    /// Without overlaps, sorted by `start_frame`.
    pub segments: Vec<AudioSegment>,
}

/// A stretch of an audio lane where one region of a file sounds (05 §10.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSegment {
    /// Absolute path of the audio file.
    pub file: String,
    /// 1 or 2.
    pub channels: i64,
    /// Song frame where the segment starts.
    pub start_frame: i64,
    /// At least 1.
    pub length_frames: i64,
    pub region_start_frame: i64,
    pub region_length_frames: i64,
    /// Frames elapsed since the start of its clip.
    pub phase_frames: i64,
}

/// Mixer controls of a track (05 §3.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelStrip {
    /// `FADER_MIN_DB`..`FADER_MAX_DB`; `FADER_MIN_DB` means gain 0.
    pub gain_db: f64,
    /// −100..100.
    pub pan: f64,
    pub mute: bool,
    pub solo: bool,
    /// In 1.0, exactly an equalizer and a reverb, in that order.
    pub effects: Vec<EffectUnit>,
}

/// An effect of a channel strip, tagged by its `type`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum EffectUnit {
    Eq(EqUnit),
    Reverb(ReverbUnit),
}

/// Three-band equalizer (05 §3.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EqUnit {
    pub enabled: bool,
    pub low_db: f64,
    pub mid_db: f64,
    pub high_db: f64,
}

/// Reverb (05 §3.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverbUnit {
    pub enabled: bool,
    pub amount: f64,
    pub size: f64,
}

/// The master channel (05 §3.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasterStrip {
    pub gain_db: f64,
}

/// The `INVALID_SNAPSHOT` error with this detail.
fn invalid<T>(detail: String) -> Result<T, EngineError> {
    Err(EngineError::InvalidSnapshot(detail))
}

/// Checks that `value` lies in `[min, max]`; NaN never does.
fn check_range(value: f64, (min, max): (f64, f64), what: &str) -> Result<(), EngineError> {
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        invalid(format!("{what} is {value}, outside {min}..{max}"))
    }
}

impl EngineSnapshot {
    /// Checks every condition of 06 §5.5, and that positions are not negative. The first broken
    /// condition gives `INVALID_SNAPSHOT` with a detail that names it.
    pub fn validate(&self) -> Result<(), EngineError> {
        if self.snapshot_version != SNAPSHOT_VERSION {
            return invalid(format!(
                "snapshotVersion is {}, not {SNAPSHOT_VERSION}",
                self.snapshot_version
            ));
        }
        let tempo = self.tempo()?;
        let Some(time_signature) = TimeSignature::from_name(&self.time_signature) else {
            return invalid(format!(
                "timeSignature {:?} is not one of the three",
                self.time_signature
            ));
        };
        if self.tracks.len() > MAX_TRACKS {
            return invalid(format!(
                "{} tracks, more than {MAX_TRACKS}",
                self.tracks.len()
            ));
        }
        for (index, track) in self.tracks.iter().enumerate() {
            if self.tracks[..index]
                .iter()
                .any(|other| other.id() == track.id())
            {
                return invalid(format!("track id {} is repeated", track.id()));
            }
            let what = format!("track {}", track.id());
            check_strip(track.strip(), &what)?;
            match track {
                SnapshotTrack::Rhythm(rhythm) => {
                    check_rhythm_track(rhythm, tempo, time_signature, &what)?;
                }
                SnapshotTrack::Audio(audio) => check_audio_track(audio, &what)?,
            }
        }
        check_range(
            self.master.gain_db,
            (FADER_MIN_DB, FADER_MAX_DB),
            "master gainDb",
        )
    }

    /// The tempo, if it is an integer between `TEMPO_MIN` and `TEMPO_MAX`.
    fn tempo(&self) -> Result<u32, EngineError> {
        let tempo = self.tempo_bpm;
        let range = f64::from(TEMPO_MIN)..=f64::from(TEMPO_MAX);
        if tempo.fract() == 0.0 && range.contains(&tempo) {
            // An integer within TEMPO_MIN..TEMPO_MAX, so the conversion is exact.
            Ok(tempo as u32)
        } else {
            invalid(format!(
                "tempoBpm is {tempo}, not an integer in {TEMPO_MIN}..{TEMPO_MAX}"
            ))
        }
    }
}

/// Volume, pan and the two effects of a channel strip.
fn check_strip(strip: &ChannelStrip, what: &str) -> Result<(), EngineError> {
    check_range(
        strip.gain_db,
        (FADER_MIN_DB, FADER_MAX_DB),
        &format!("{what}: gainDb"),
    )?;
    check_range(strip.pan, PAN_RANGE, &format!("{what}: pan"))?;
    let [EffectUnit::Eq(eq), EffectUnit::Reverb(reverb)] = strip.effects.as_slice() else {
        return invalid(format!("{what}: effects are not [eq, reverb]"));
    };
    let eq_range = (EQ_GAIN_MIN_DB, EQ_GAIN_MAX_DB);
    check_range(eq.low_db, eq_range, &format!("{what}: eq lowDb"))?;
    check_range(eq.mid_db, eq_range, &format!("{what}: eq midDb"))?;
    check_range(eq.high_db, eq_range, &format!("{what}: eq highDb"))?;
    check_range(
        reverb.amount,
        REVERB_RANGE,
        &format!("{what}: reverb amount"),
    )?;
    check_range(reverb.size, REVERB_RANGE, &format!("{what}: reverb size"))
}

/// Rows, kit, patterns and segments of a rhythm track.
fn check_rhythm_track(
    track: &SnapshotRhythmTrack,
    tempo: u32,
    time_signature: TimeSignature,
    what: &str,
) -> Result<(), EngineError> {
    if track.rows.len() != KIT_ROWS {
        return invalid(format!("{what}: {} rows, not {KIT_ROWS}", track.rows.len()));
    }
    for (row, settings) in track.rows.iter().enumerate() {
        check_range(
            settings.gain_db,
            (ROW_GAIN_MIN_DB, ROW_GAIN_MAX_DB),
            &format!("{what}: row {row} gainDb"),
        )?;
    }
    if kit_index(&track.kit_id).is_none() {
        return invalid(format!("{what}: kitId {:?} is not a kit", track.kit_id));
    }
    if track.patterns.len() > MAX_PATTERNS_PER_TRACK {
        return invalid(format!(
            "{what}: {} patterns, more than {MAX_PATTERNS_PER_TRACK}",
            track.patterns.len()
        ));
    }
    for pattern in &track.patterns {
        check_pattern(
            pattern,
            time_signature,
            &format!("{what}: pattern {}", pattern.id),
        )?;
    }
    for segment in &track.segments {
        if !track.patterns.iter().any(|p| p.id == segment.pattern_id) {
            return invalid(format!(
                "{what}: a segment refers to pattern {}, which is not in the track",
                segment.pattern_id
            ));
        }
        if segment.phase_steps < 0 {
            return invalid(format!("{what}: a segment has phaseSteps < 0"));
        }
    }
    check_spans(
        track
            .segments
            .iter()
            .map(|segment| (segment.start_step, segment.length_steps)),
        |end_step| step_to_frame(end_step, tempo),
        what,
    )
}

/// Length and levels of a pattern.
fn check_pattern(
    pattern: &SnapshotPattern,
    time_signature: TimeSignature,
    what: &str,
) -> Result<(), EngineError> {
    let steps_per_bar = time_signature.steps_per_bar();
    let allowed = PATTERN_BARS.map(|bars| bars * steps_per_bar);
    let length_ok = u64::try_from(pattern.length_steps)
        .map(|length| allowed.contains(&length))
        .unwrap_or(false);
    if !length_ok {
        return invalid(format!(
            "{what}: lengthSteps {} is not one of {allowed:?}",
            pattern.length_steps
        ));
    }
    if pattern.steps.len() != KIT_ROWS {
        return invalid(format!(
            "{what}: {} rows of steps, not {KIT_ROWS}",
            pattern.steps.len()
        ));
    }
    for (row, levels) in pattern.steps.iter().enumerate() {
        if i64::try_from(levels.len()) != Ok(pattern.length_steps) {
            return invalid(format!(
                "{what}: row {row} has {} steps, not {}",
                levels.len(),
                pattern.length_steps
            ));
        }
        if let Some(level) = levels
            .iter()
            .find(|level| !(0..=MAX_LEVEL).contains(*level))
        {
            return invalid(format!(
                "{what}: row {row} has level {level}, outside 0..{MAX_LEVEL}"
            ));
        }
    }
    Ok(())
}

/// Segments of an audio track.
fn check_audio_track(track: &SnapshotAudioTrack, what: &str) -> Result<(), EngineError> {
    for segment in &track.segments {
        let file = format!("{what}: segment of {}", segment.file);
        if !matches!(segment.channels, 1 | 2) {
            return invalid(format!("{file}: channels is {}", segment.channels));
        }
        if segment.region_length_frames < 1 {
            return invalid(format!("{file}: regionLengthFrames < 1"));
        }
        if segment.phase_frames < 0 {
            return invalid(format!("{file}: phaseFrames < 0"));
        }
        if segment.region_start_frame < 0 {
            return invalid(format!("{file}: regionStartFrame < 0"));
        }
    }
    check_spans(
        track
            .segments
            .iter()
            .map(|segment| (segment.start_frame, segment.length_frames)),
        |end_frame| end_frame,
        what,
    )
}

/// Checks the segments of a lane, given as `(start, length)` in their own unit: none starts
/// before 0, each is at least 1 long, they are sorted by start and do not overlap, and each ends
/// by `MAX_SONG_FRAMES` once its end is converted to frames with `end_frame`.
fn check_spans(
    spans: impl Iterator<Item = (i64, i64)>,
    end_frame: impl Fn(u64) -> u64,
    what: &str,
) -> Result<(), EngineError> {
    let mut previous: Option<(i64, i64)> = None;
    for (index, (start, length)) in spans.enumerate() {
        let segment = format!("{what}: segment {index}");
        if start < 0 {
            return invalid(format!("{segment} starts at {start}, before 0"));
        }
        if length < 1 {
            return invalid(format!("{segment} has length {length}"));
        }
        if let Some((previous_start, previous_end)) = previous {
            if start < previous_start {
                return invalid(format!("{segment} starts before the previous one"));
            }
            if start < previous_end {
                return invalid(format!("{segment} overlaps the previous one"));
            }
        }
        let end = start.saturating_add(length);
        // `end` is positive: start ≥ 0 and length ≥ 1.
        let end_frames = end_frame(u64::try_from(end).unwrap_or(u64::MAX));
        if end_frames > MAX_SONG_FRAMES {
            return invalid(format!(
                "{segment} ends at frame {end_frames}, after {MAX_SONG_FRAMES}"
            ));
        }
        previous = Some((start, end));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    /// The example of 05 §10.4, with the `strip`, `rows` and `steps` of the project of 05 §5.3.
    const EXAMPLE: &str = r#"{
      "snapshotVersion": 1,
      "tempoBpm": 100,
      "timeSignature": "4/4",
      "tracks": [
        {
          "id": "trk-1",
          "type": "rhythm",
          "strip": {
            "gainDb": -3,
            "pan": 0,
            "mute": false,
            "solo": false,
            "effects": [
              { "type": "eq", "enabled": false, "lowDb": 0, "midDb": 0, "highDb": 0 },
              { "type": "reverb", "enabled": false, "amount": 20, "size": 50 }
            ]
          },
          "kitId": "acoustic",
          "rows": [
            { "gainDb": 0, "mute": false },
            { "gainDb": 0, "mute": false },
            { "gainDb": 0, "mute": false },
            { "gainDb": -6, "mute": false },
            { "gainDb": 0, "mute": false },
            { "gainDb": 0, "mute": false },
            { "gainDb": 0, "mute": false },
            { "gainDb": 0, "mute": false }
          ],
          "patterns": [
            {
              "id": "pat-2",
              "lengthSteps": 16,
              "steps": [
                [2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
              ]
            }
          ],
          "segments": [ { "patternId": "pat-2", "startStep": 0, "lengthSteps": 256, "phaseSteps": 0 } ]
        },
        {
          "id": "trk-4",
          "type": "audio",
          "strip": {
            "gainDb": 0,
            "pan": 0,
            "mute": false,
            "solo": false,
            "effects": [
              { "type": "eq", "enabled": false, "lowDb": 0, "midDb": 0, "highDb": 0 },
              { "type": "reverb", "enabled": true, "amount": 25, "size": 60 }
            ]
          },
          "segments": [
            {
              "file": "C:\\Users\\ana\\Documents\\Carabo\\Demo\\audio\\take-0001.wav",
              "channels": 1,
              "startFrame": 460800,
              "lengthFrames": 573600,
              "regionStartFrame": 2400,
              "regionLengthFrames": 573600,
              "phaseFrames": 0
            }
          ]
        }
      ],
      "master": { "gainDb": 0 }
    }"#;

    fn example() -> EngineSnapshot {
        serde_json::from_str(EXAMPLE).expect("the example of 05 §10.4 deserializes")
    }

    fn rhythm(snapshot: &mut EngineSnapshot) -> &mut SnapshotRhythmTrack {
        match &mut snapshot.tracks[0] {
            SnapshotTrack::Rhythm(track) => track,
            SnapshotTrack::Audio(_) => unreachable!("the first track of the example is rhythm"),
        }
    }

    fn audio(snapshot: &mut EngineSnapshot) -> &mut SnapshotAudioTrack {
        match &mut snapshot.tracks[1] {
            SnapshotTrack::Audio(track) => track,
            SnapshotTrack::Rhythm(_) => unreachable!("the second track of the example is audio"),
        }
    }

    fn strip(snapshot: &mut EngineSnapshot) -> &mut ChannelStrip {
        &mut rhythm(snapshot).strip
    }

    fn eq(snapshot: &mut EngineSnapshot) -> &mut EqUnit {
        match &mut strip(snapshot).effects[0] {
            EffectUnit::Eq(eq) => eq,
            EffectUnit::Reverb(_) => unreachable!("the first effect is the equalizer"),
        }
    }

    fn reverb(snapshot: &mut EngineSnapshot) -> &mut ReverbUnit {
        match &mut strip(snapshot).effects[1] {
            EffectUnit::Reverb(reverb) => reverb,
            EffectUnit::Eq(_) => unreachable!("the second effect is the reverb"),
        }
    }

    fn audio_segment(snapshot: &mut EngineSnapshot) -> &mut AudioSegment {
        &mut audio(snapshot).segments[0]
    }

    fn rhythm_segment(snapshot: &mut EngineSnapshot) -> &mut RhythmSegment {
        &mut rhythm(snapshot).segments[0]
    }

    /// The example, changed by `change`.
    fn changed(change: impl FnOnce(&mut EngineSnapshot)) -> EngineSnapshot {
        let mut snapshot = example();
        change(&mut snapshot);
        snapshot
    }

    /// The example, changed by `change`, is rejected with `INVALID_SNAPSHOT`.
    fn rejects(change: impl FnOnce(&mut EngineSnapshot)) {
        let result = changed(change).validate();
        assert!(
            matches!(result, Err(EngineError::InvalidSnapshot(_))),
            "{result:?}"
        );
    }

    /// The example, changed by `change`, is still valid.
    fn accepts(change: impl FnOnce(&mut EngineSnapshot)) {
        assert_eq!(changed(change).validate(), Ok(()));
    }

    /// A second rhythm segment of the example's pattern.
    fn second_rhythm_segment(start_step: i64, length_steps: i64) -> RhythmSegment {
        RhythmSegment {
            pattern_id: "pat-2".to_string(),
            start_step,
            length_steps,
            phase_steps: 0,
        }
    }

    // ---- The example ----

    #[test]
    fn the_example_of_05_10_4_deserializes_and_passes_the_validation() {
        let mut snapshot = example();
        assert_eq!(snapshot.validate(), Ok(()));
        assert_eq!(snapshot.tempo_bpm, 100.0);
        assert_eq!(snapshot.time_signature, "4/4");
        assert_eq!(rhythm(&mut snapshot).kit_id, "acoustic");
        assert_eq!(rhythm(&mut snapshot).rows[3].gain_db, -6.0);
        assert_eq!(rhythm(&mut snapshot).patterns[0].steps[3][2], 2);
        assert_eq!(rhythm_segment(&mut snapshot).length_steps, 256);
        assert_eq!(audio_segment(&mut snapshot).start_frame, 460_800);
        assert_eq!(reverb(&mut snapshot).amount, 20.0);
    }

    #[test]
    fn a_snapshot_without_tracks_is_valid() {
        accepts(|s| s.tracks.clear());
    }

    #[test]
    fn values_that_the_types_of_the_interface_allow_reach_the_validation() {
        let mut json: Value = serde_json::from_str(EXAMPLE).expect("the example is JSON");
        json["tempoBpm"] = 120.5.into();
        json["tracks"][0]["kitId"] = "jazz".into();
        json["tracks"][1]["segments"][0]["phaseFrames"] = (-1).into();
        let snapshot: EngineSnapshot =
            serde_json::from_value(json).expect("out-of-range values still deserialize");
        assert!(matches!(
            snapshot.validate(),
            Err(EngineError::InvalidSnapshot(_))
        ));
    }

    // ---- 06 §5.5: version, tempo, time signature and tracks ----

    #[test]
    fn rejects_a_snapshot_version_other_than_1() {
        rejects(|s| s.snapshot_version = 2);
    }

    #[test]
    fn rejects_a_tempo_that_is_not_an_integer() {
        rejects(|s| s.tempo_bpm = 120.5);
    }

    #[test]
    fn rejects_a_tempo_below_tempo_min() {
        rejects(|s| s.tempo_bpm = f64::from(TEMPO_MIN) - 1.0);
    }

    #[test]
    fn rejects_a_tempo_above_tempo_max() {
        rejects(|s| s.tempo_bpm = f64::from(TEMPO_MAX) + 1.0);
    }

    #[test]
    fn accepts_tempo_min_and_tempo_max() {
        accepts(|s| s.tempo_bpm = f64::from(TEMPO_MIN));
        accepts(|s| s.tempo_bpm = f64::from(TEMPO_MAX));
    }

    #[test]
    fn rejects_an_unknown_time_signature() {
        rejects(|s| s.time_signature = "5/4".to_string());
    }

    #[test]
    fn rejects_more_than_max_tracks_tracks() {
        let track = |n: usize| {
            let mut track = example().tracks[1].clone();
            if let SnapshotTrack::Audio(audio) = &mut track {
                audio.id = format!("trk-{n}");
            }
            track
        };
        accepts(|s| s.tracks = (0..MAX_TRACKS).map(track).collect());
        rejects(|s| s.tracks = (0..=MAX_TRACKS).map(track).collect());
    }

    #[test]
    fn rejects_a_repeated_track_id() {
        rejects(|s| audio(s).id = "trk-1".to_string());
    }

    // ---- 06 §5.5: channel strips ----

    #[test]
    fn rejects_a_track_gain_outside_the_fader_range() {
        accepts(|s| strip(s).gain_db = FADER_MIN_DB);
        accepts(|s| strip(s).gain_db = FADER_MAX_DB);
        rejects(|s| strip(s).gain_db = FADER_MIN_DB - 0.5);
        rejects(|s| strip(s).gain_db = FADER_MAX_DB + 0.5);
    }

    #[test]
    fn rejects_a_master_gain_outside_the_fader_range() {
        rejects(|s| s.master.gain_db = FADER_MIN_DB - 0.5);
        rejects(|s| s.master.gain_db = FADER_MAX_DB + 0.5);
    }

    #[test]
    fn rejects_a_pan_outside_minus_100_to_100() {
        accepts(|s| strip(s).pan = -100.0);
        accepts(|s| strip(s).pan = 100.0);
        rejects(|s| strip(s).pan = -101.0);
        rejects(|s| strip(s).pan = 101.0);
    }

    #[test]
    fn rejects_effects_other_than_an_equalizer_and_a_reverb_in_that_order() {
        rejects(|s| strip(s).effects.reverse());
        rejects(|s| strip(s).effects.truncate(1));
        rejects(|s| {
            let eq = strip(s).effects[0].clone();
            strip(s).effects.push(eq);
        });
    }

    #[test]
    fn rejects_an_equalizer_band_outside_its_range() {
        accepts(|s| eq(s).low_db = EQ_GAIN_MIN_DB);
        accepts(|s| eq(s).high_db = EQ_GAIN_MAX_DB);
        rejects(|s| eq(s).low_db = EQ_GAIN_MIN_DB - 0.5);
        rejects(|s| eq(s).mid_db = EQ_GAIN_MAX_DB + 0.5);
        rejects(|s| eq(s).high_db = EQ_GAIN_MAX_DB + 0.5);
    }

    #[test]
    fn rejects_a_reverb_amount_or_size_outside_0_to_100() {
        accepts(|s| reverb(s).amount = 0.0);
        accepts(|s| reverb(s).size = 100.0);
        rejects(|s| reverb(s).amount = -1.0);
        rejects(|s| reverb(s).amount = 101.0);
        rejects(|s| reverb(s).size = 101.0);
    }

    // ---- 06 §5.5: rhythm tracks ----

    #[test]
    fn rejects_a_rhythm_track_without_8_rows() {
        rejects(|s| {
            rhythm(s).rows.pop();
        });
    }

    #[test]
    fn rejects_a_row_gain_outside_its_range() {
        accepts(|s| rhythm(s).rows[0].gain_db = ROW_GAIN_MIN_DB);
        accepts(|s| rhythm(s).rows[0].gain_db = ROW_GAIN_MAX_DB);
        rejects(|s| rhythm(s).rows[0].gain_db = ROW_GAIN_MIN_DB - 1.0);
        rejects(|s| rhythm(s).rows[7].gain_db = ROW_GAIN_MAX_DB + 1.0);
    }

    #[test]
    fn rejects_a_kit_that_is_not_one_of_the_kits() {
        accepts(|s| rhythm(s).kit_id = "electronic".to_string());
        accepts(|s| rhythm(s).kit_id = "percussion".to_string());
        rejects(|s| rhythm(s).kit_id = "jazz".to_string());
    }

    #[test]
    fn rejects_more_than_max_patterns_per_track_patterns() {
        let patterns = |count: usize| {
            let pattern = example_pattern();
            (0..count)
                .map(|n| SnapshotPattern {
                    id: if n == 0 {
                        "pat-2".to_string()
                    } else {
                        format!("pat-x{n}")
                    },
                    ..pattern.clone()
                })
                .collect::<Vec<_>>()
        };
        accepts(|s| rhythm(s).patterns = patterns(MAX_PATTERNS_PER_TRACK));
        rejects(|s| rhythm(s).patterns = patterns(MAX_PATTERNS_PER_TRACK + 1));
    }

    fn example_pattern() -> SnapshotPattern {
        rhythm(&mut example()).patterns[0].clone()
    }

    #[test]
    fn rejects_a_pattern_without_8_rows_of_steps() {
        rejects(|s| {
            rhythm(s).patterns[0].steps.pop();
        });
    }

    #[test]
    fn rejects_a_pattern_row_without_length_steps_levels() {
        rejects(|s| {
            rhythm(s).patterns[0].steps[5].pop();
        });
        rejects(|s| rhythm(s).patterns[0].steps[0].push(0));
    }

    #[test]
    fn rejects_a_level_outside_0_to_3() {
        accepts(|s| rhythm(s).patterns[0].steps[2][1] = 3);
        rejects(|s| rhythm(s).patterns[0].steps[2][1] = 4);
        rejects(|s| rhythm(s).patterns[0].steps[2][1] = -1);
    }

    /// An empty pattern `pat-2` of `length_steps` steps.
    fn pattern_of(length_steps: i64) -> SnapshotPattern {
        let length = usize::try_from(length_steps).expect("a test length is not negative");
        SnapshotPattern {
            id: "pat-2".to_string(),
            length_steps,
            steps: vec![vec![0; length]; KIT_ROWS],
        }
    }

    #[test]
    fn rejects_a_pattern_length_that_is_not_1_2_or_4_bars() {
        for steps in [16, 32, 64] {
            accepts(|s| rhythm(s).patterns[0] = pattern_of(steps));
        }
        for steps in [0, 12, 15, 48, 128] {
            rejects(|s| rhythm(s).patterns[0] = pattern_of(steps));
        }
    }

    #[test]
    fn pattern_lengths_follow_the_steps_per_bar_of_the_time_signature() {
        for signature in ["3/4", "6/8"] {
            for steps in [12, 24, 48] {
                accepts(|s| {
                    s.time_signature = signature.to_string();
                    rhythm(s).patterns[0] = pattern_of(steps);
                });
            }
            rejects(|s| s.time_signature = signature.to_string());
        }
    }

    #[test]
    fn rejects_a_segment_whose_pattern_is_not_in_the_track() {
        rejects(|s| rhythm_segment(s).pattern_id = "pat-9".to_string());
    }

    // ---- 06 §5.5: segments of every track ----

    #[test]
    fn rejects_rhythm_segments_that_are_not_sorted_by_start() {
        rejects(|s| {
            rhythm_segment(s).start_step = 300;
            rhythm(s).segments.push(second_rhythm_segment(0, 16));
        });
    }

    #[test]
    fn rejects_overlapping_rhythm_segments() {
        accepts(|s| rhythm(s).segments.push(second_rhythm_segment(256, 16)));
        rejects(|s| rhythm(s).segments.push(second_rhythm_segment(255, 16)));
    }

    #[test]
    fn rejects_a_rhythm_segment_shorter_than_1_step() {
        rejects(|s| rhythm_segment(s).length_steps = 0);
    }

    #[test]
    fn rejects_a_rhythm_segment_that_ends_after_max_song_frames() {
        // At 100 bpm a step is 7 200 frames, so MAX_SONG_FRAMES is exactly step 4 000.
        accepts(|s| rhythm_segment(s).length_steps = 4_000);
        rejects(|s| rhythm_segment(s).length_steps = 4_001);
    }

    #[test]
    fn rejects_audio_segments_that_are_not_sorted_by_start() {
        rejects(|s| {
            let mut earlier = audio_segment(s).clone();
            earlier.start_frame = 0;
            earlier.length_frames = 1_000;
            audio(s).segments.push(earlier);
        });
    }

    #[test]
    fn rejects_overlapping_audio_segments() {
        let next = |start_frame: i64| {
            move |s: &mut EngineSnapshot| {
                let mut next = audio_segment(s).clone();
                next.start_frame = start_frame;
                audio(s).segments.push(next);
            }
        };
        // The segment of the example covers [460 800, 1 034 400).
        accepts(next(1_034_400));
        rejects(next(1_034_399));
    }

    #[test]
    fn rejects_an_audio_segment_shorter_than_1_frame() {
        rejects(|s| audio_segment(s).length_frames = 0);
    }

    #[test]
    fn rejects_an_audio_segment_that_ends_after_max_song_frames() {
        let max = i64::try_from(MAX_SONG_FRAMES).expect("MAX_SONG_FRAMES fits in i64");
        accepts(|s| audio_segment(s).start_frame = max - 573_600);
        rejects(|s| audio_segment(s).start_frame = max - 573_599);
    }

    // ---- 06 §5.5: audio segments ----

    #[test]
    fn rejects_an_audio_segment_with_channels_other_than_1_or_2() {
        accepts(|s| audio_segment(s).channels = 2);
        rejects(|s| audio_segment(s).channels = 0);
        rejects(|s| audio_segment(s).channels = 3);
    }

    #[test]
    fn rejects_a_region_shorter_than_1_frame() {
        rejects(|s| audio_segment(s).region_length_frames = 0);
    }

    #[test]
    fn rejects_a_negative_phase_frames() {
        rejects(|s| audio_segment(s).phase_frames = -1);
    }

    #[test]
    fn rejects_a_negative_region_start_frame() {
        accepts(|s| audio_segment(s).region_start_frame = 0);
        rejects(|s| audio_segment(s).region_start_frame = -1);
    }

    // ---- Positions before the start of the song ----

    #[test]
    fn rejects_segments_that_start_before_0() {
        rejects(|s| rhythm_segment(s).start_step = -1);
        rejects(|s| audio_segment(s).start_frame = -1);
    }

    #[test]
    fn rejects_a_negative_phase_steps() {
        rejects(|s| rhythm_segment(s).phase_steps = -1);
    }
}
