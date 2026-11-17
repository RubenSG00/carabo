//! Shared helpers for the engine's integration tests (08 §3.4, §8.1): snapshot builders, the
//! test kit and the harness that drives a `Processor` without an audio device.

// Each test file uses only part of this module.
#![allow(dead_code)]

use std::fs;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

use carabo_engine::constants::KIT_ROWS;
use carabo_engine::kits::{load_kits, Kits, KIT_IDS, KIT_ROLES};
use carabo_engine::snapshot::{
    ChannelStrip, EffectUnit, EngineSnapshot, EqUnit, MasterStrip, ReverbUnit, RowSettings,
    SnapshotPattern, SnapshotRhythmTrack, SnapshotTrack, SNAPSHOT_VERSION,
};
use carabo_engine::testing::{
    self, CommandProducer, PlayMode, Preparer, Processor, RtCommand, SharedState,
};
use carabo_engine::Telemetry;
use hound::{SampleFormat, WavSpec, WavWriter};
use serde_json::json;

/// Row of the closed hi-hat in every kit (05 §8.1).
pub const HAT_CLOSED: usize = 3;
/// Row of the open hi-hat in every kit (05 §8.1).
pub const HAT_OPEN: usize = 4;

/// Builder of snapshots for the tests (08 §3.4).
pub struct SnapshotBuilder {
    snapshot: EngineSnapshot,
}

/// A valid snapshot without tracks: 120, 4/4 and the master at 0 dB.
pub fn snapshot() -> SnapshotBuilder {
    SnapshotBuilder {
        snapshot: EngineSnapshot {
            snapshot_version: SNAPSHOT_VERSION,
            tempo_bpm: 120.0,
            time_signature: "4/4".to_string(),
            tracks: Vec::new(),
            master: MasterStrip { gain_db: 0.0 },
        },
    }
}

impl SnapshotBuilder {
    pub fn tempo(mut self, tempo_bpm: u32) -> Self {
        self.snapshot.tempo_bpm = f64::from(tempo_bpm);
        self
    }

    pub fn time_signature(mut self, time_signature: &str) -> Self {
        self.snapshot.time_signature = time_signature.to_string();
        self
    }

    /// Adds a rhythm track at the end of the list.
    pub fn rhythm(mut self, track: RhythmTrackBuilder) -> Self {
        self.snapshot
            .tracks
            .push(SnapshotTrack::Rhythm(track.track));
        self
    }

    pub fn build(self) -> EngineSnapshot {
        self.snapshot
    }
}

/// A channel strip with its initial values: 0 dB, centred, without mute or solo, and both
/// effects off (05 §3.1).
pub fn strip() -> ChannelStrip {
    ChannelStrip {
        gain_db: 0.0,
        pan: 0.0,
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

/// Builder of a rhythm track for [`SnapshotBuilder::rhythm`].
pub struct RhythmTrackBuilder {
    track: SnapshotRhythmTrack,
}

/// A rhythm track with the acoustic kit, its rows at 0 dB and its channel with the initial
/// values, without patterns or segments.
pub fn rhythm_track(id: &str) -> RhythmTrackBuilder {
    RhythmTrackBuilder {
        track: SnapshotRhythmTrack {
            id: id.to_string(),
            strip: strip(),
            kit_id: "acoustic".to_string(),
            rows: (0..KIT_ROWS)
                .map(|_| RowSettings {
                    gain_db: 0.0,
                    mute: false,
                })
                .collect(),
            patterns: Vec::new(),
            segments: Vec::new(),
        },
    }
}

impl RhythmTrackBuilder {
    /// Adds a pattern of `length_steps` steps with `hits`, each one `(row, step, level)`.
    pub fn pattern(mut self, id: &str, length_steps: usize, hits: &[(usize, usize, i64)]) -> Self {
        let mut steps = vec![vec![0; length_steps]; KIT_ROWS];
        for &(row, step, level) in hits {
            steps[row][step] = level;
        }
        self.track.patterns.push(SnapshotPattern {
            id: id.to_string(),
            length_steps: length_steps as i64,
            steps,
        });
        self
    }

    pub fn kit(mut self, kit_id: &str) -> Self {
        self.track.kit_id = kit_id.to_string();
        self
    }

    pub fn row_gain_db(mut self, row: usize, gain_db: f64) -> Self {
        self.track.rows[row].gain_db = gain_db;
        self
    }

    pub fn row_mute(mut self, row: usize) -> Self {
        self.track.rows[row].mute = true;
        self
    }

    /// Volume of the channel, in dB.
    pub fn gain_db(mut self, gain_db: f64) -> Self {
        self.track.strip.gain_db = gain_db;
        self
    }

    pub fn pan(mut self, pan: f64) -> Self {
        self.track.strip.pan = pan;
        self
    }

    pub fn mute(mut self) -> Self {
        self.track.strip.mute = true;
        self
    }

    pub fn solo(mut self) -> Self {
        self.track.strip.solo = true;
        self
    }
}

/// The test kit of 08 §8.1: the same 8 sounds in the three kits, written to a temporary folder
/// and loaded as `Engine::start` loads the real ones.
#[derive(Clone)]
pub struct TestKit {
    sounds: Vec<Vec<f32>>,
    hat_choke: bool,
}

impl TestKit {
    /// 8 sounds of a single frame of value 1,0, without choke groups: a hit is an impulse and
    /// its frame can be read directly (08 §8.1).
    pub fn impulses() -> Self {
        Self {
            sounds: vec![vec![1.0]; KIT_ROWS],
            hat_choke: false,
        }
    }

    /// Replaces the sound of `row`.
    pub fn sound(mut self, row: usize, samples: Vec<f32>) -> Self {
        self.sounds[row] = samples;
        self
    }

    /// Replaces the sound of every row.
    pub fn all_sounds(mut self, samples: Vec<f32>) -> Self {
        self.sounds = vec![samples; KIT_ROWS];
        self
    }

    /// Puts both hi-hats in one choke group, as in the real kits (05 §8.3).
    pub fn hat_choke(mut self) -> Self {
        self.hat_choke = true;
        self
    }

    /// Writes the three kits to a temporary folder and loads them (06 §8.4).
    pub fn load(&self) -> Arc<Kits> {
        let dir = tempfile::tempdir().expect("a temporary folder");
        for kit in KIT_IDS {
            self.write_kit(&dir.path().join(kit));
        }
        Arc::new(load_kits(dir.path()).expect("the test kit loads"))
    }

    fn write_kit(&self, dir: &Path) {
        fs::create_dir(dir).expect("the kit folder is created");
        let sounds: Vec<_> = KIT_ROLES
            .iter()
            .enumerate()
            .map(|(row, role)| {
                let file = format!("{role}.wav");
                write_wav(&dir.join(&file), &self.sounds[row]);
                let hat = self.hat_choke && (row == HAT_CLOSED || row == HAT_OPEN);
                json!({ "role": role, "file": file, "chokeGroup": hat.then_some("hat") })
            })
            .collect();
        let kit = json!({ "sounds": sounds });
        fs::write(dir.join("kit.json"), kit.to_string()).expect("kit.json is written");
    }
}

/// Writes a mono WAV at 48 000 Hz in 32-bit floating point, so that the samples stay exact.
fn write_wav(path: &Path, samples: &[f32]) {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 48_000,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    let mut writer = WavWriter::create(path, spec).expect("the WAV file is created");
    for sample in samples {
        writer.write_sample(*sample).expect("a sample is written");
    }
    writer.finalize().expect("the WAV file is finished");
}

/// Drives a `Processor` directly, as the output thread does (08 §8.1).
pub struct Harness {
    processor: Processor,
    commands: CommandProducer,
    shared: Arc<SharedState>,
    /// What the control thread keeps of the snapshots (06 §5.3, §5.4).
    preparer: Preparer,
}

impl Harness {
    /// Prepares `snapshot` with the kit of impulses and creates a `Processor` in `Stopped`.
    pub fn new(snapshot: &EngineSnapshot) -> Self {
        Self::with_kit(snapshot, &TestKit::impulses())
    }

    /// Prepares `snapshot` with `kit` and creates a `Processor` in `Stopped`.
    pub fn with_kit(snapshot: &EngineSnapshot, kit: &TestKit) -> Self {
        let (processor, commands, shared) = testing::processor(kit.load());
        let mut harness = Self {
            processor,
            commands,
            shared,
            preparer: Preparer::new(),
        };
        harness.set_project(snapshot);
        harness
    }

    /// Prepares a new snapshot and sends it, as `EngineHandle::set_project` does (06 §5.1).
    pub fn set_project(&mut self, snapshot: &EngineSnapshot) {
        snapshot
            .validate()
            .expect("the tests only use valid snapshots");
        let project = self.preparer.prepare(snapshot.clone());
        self.send(RtCommand::SetProject(Arc::new(project)));
    }

    /// Plays a pattern in pattern mode, as `EngineHandle::play` does (06 §5.1).
    pub fn play_pattern(&mut self, track_id: &str, pattern_id: &str) {
        let (slot, pattern_index) = self
            .preparer
            .pattern(track_id, pattern_id)
            .expect("the pattern is in the snapshot");
        self.send(RtCommand::Play(PlayMode::Pattern {
            slot,
            pattern_index,
        }));
    }

    /// Plays a sound of a rhythm track, as `EngineHandle::audition` does (06 §5.1).
    pub fn audition(&mut self, track_id: &str, row: usize) {
        let slot = self
            .preparer
            .rhythm_slot(track_id)
            .expect("the rhythm track is in the snapshot");
        self.send(RtCommand::Audition { slot, row });
    }

    pub fn stop(&mut self) {
        self.send(RtCommand::Stop);
    }

    /// Queues a command; the `Processor` applies it at the start of the next block (06 §7).
    pub fn send(&mut self, command: RtCommand) {
        assert!(
            self.commands.push(command).is_ok(),
            "the command queue is full"
        );
    }

    /// Computes `frames` frames in blocks of `block` frames (the last one, of what is left) and
    /// returns them as interleaved stereo.
    pub fn run(&mut self, frames: usize, block: usize) -> Vec<f32> {
        assert!(block > 0);
        let mut out = vec![0.0; 2 * frames];
        let mut done = 0;
        while done < frames {
            let n = block.min(frames - done);
            self.processor
                .process(&mut out[2 * done..2 * (done + n)], n);
            done += n;
        }
        out
    }

    /// The telemetry as `EngineHandle::telemetry()` would build it.
    pub fn telemetry(&self) -> Telemetry {
        self.shared.telemetry(&self.preparer.track_slots())
    }

    /// The engine clock (06 §6.2).
    pub fn engine_clock(&self) -> u64 {
        self.shared.engine_clock()
    }

    /// Samples clipped by the master since the `Processor` was created (06 §10.3).
    pub fn clipped_samples(&self) -> u64 {
        self.processor.clipped_samples()
    }
}

/// Frame of step `step` at `tempo_bpm`, with the formula of 05 §2.2 and 06 §8.1.
pub fn step_frame(step: u64, tempo_bpm: u32) -> usize {
    (step as f64 * 720_000.0 / f64::from(tempo_bpm)).round() as usize
}

/// Linear gain of `db` decibels (00 §3.5).
pub fn db_gain(db: f64) -> f64 {
    10f64.powf(db / 20.0)
}

/// Sample of `channel` (0 left, 1 right) at `frame` of an interleaved stereo buffer.
pub fn sample(out: &[f32], channel: usize, frame: usize) -> f32 {
    out[2 * frame + channel]
}

/// Number of frames of an interleaved stereo buffer.
pub fn frames(out: &[f32]) -> usize {
    out.len() / 2
}

/// First frame at or after `from` where either channel is not 0.
pub fn first_nonzero_frame(out: &[f32], from: usize) -> Option<usize> {
    (from..frames(out)).find(|&f| sample(out, 0, f) != 0.0 || sample(out, 1, f) != 0.0)
}

/// Every frame where either channel is not 0, with the value of each channel.
pub fn nonzero_frames(out: &[f32]) -> Vec<(usize, [f32; 2])> {
    (0..frames(out))
        .filter(|&f| sample(out, 0, f) != 0.0 || sample(out, 1, f) != 0.0)
        .map(|f| (f, [sample(out, 0, f), sample(out, 1, f)]))
        .collect()
}

/// Root mean square of `channel` over `range` of frames.
pub fn rms(out: &[f32], channel: usize, range: Range<usize>) -> f64 {
    let n = range.len() as f64;
    let sum: f64 = range
        .map(|f| f64::from(sample(out, channel, f)).powi(2))
        .sum();
    (sum / n).sqrt()
}

/// Largest absolute value of `channel` over `range` of frames.
pub fn peak(out: &[f32], channel: usize, range: Range<usize>) -> f32 {
    range
        .map(|f| sample(out, channel, f).abs())
        .fold(0.0, f32::max)
}

/// Amplitude of a tone of `hz` in `channel` over `range` of frames, by correlation with a sine
/// and a cosine. Exact for a whole number of periods.
pub fn tone_level(out: &[f32], hz: f64, channel: usize, range: Range<usize>) -> f64 {
    let n = range.len() as f64;
    let (mut sin_sum, mut cos_sum) = (0.0, 0.0);
    for f in range {
        let x = f64::from(sample(out, channel, f));
        let phase = 2.0 * std::f64::consts::PI * hz * f as f64 / 48_000.0;
        sin_sum += x * phase.sin();
        cos_sum += x * phase.cos();
    }
    2.0 * (sin_sum * sin_sum + cos_sum * cos_sum).sqrt() / n
}

/// Largest difference between two consecutive samples of the same channel over `range` of
/// frames.
pub fn max_step(out: &[f32], range: Range<usize>) -> f32 {
    let mut largest: f32 = 0.0;
    for f in range.start + 1..range.end {
        for channel in 0..2 {
            let step = (sample(out, channel, f) - sample(out, channel, f - 1)).abs();
            largest = largest.max(step);
        }
    }
    largest
}
