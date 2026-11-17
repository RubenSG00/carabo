//! Hits and pattern mode (T-007): the rows of 08 §8.2 «Frame exacto de cada golpe», «Volumen y
//! silencio de fila», «Grupo de corte», «Límite de voces», «Escucha» and «Parar», played in
//! pattern mode, plus the telemetry and the `EngineHandle` methods of pattern mode.

mod common;

use std::f64::consts::PI;
use std::path::PathBuf;

use carabo_engine::constants::{KIT_ROWS, LEVEL_GAINS, MAX_VOICES, MICROFADE_FRAMES};
use carabo_engine::testing;
use carabo_engine::{EngineConfig, EngineError, PlayRequest, TelemetryMode, TelemetryState};
use common::{
    db_gain, first_nonzero_frame, frames, max_step, nonzero_frames, rhythm_track, sample, snapshot,
    step_frame, Harness, TestKit, HAT_CLOSED, HAT_OPEN,
};

/// Gain of a hit of level 2, the only level besides 0 that the 1.0 uses (00 §7).
const NORMAL: f32 = LEVEL_GAINS[2];
/// Frames of a fade (06 §6.1, §8.2).
const FADE: usize = MICROFADE_FRAMES as usize;
/// Frames of a step at 120 quarter notes per minute.
const STEP_AT_120: usize = 6_000;
/// Frames of a 4/4 bar at 120 quarter notes per minute.
const BAR_AT_120: usize = 16 * STEP_AT_120;

fn kits_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/kits")
}

/// A sound of `frames` frames of constant value `value`.
fn constant(value: f32, frames: usize) -> Vec<f32> {
    vec![value; frames]
}

/// Asserts that `out` is `expected(frame)` in both channels at every frame, within `tolerance`.
fn assert_output(out: &[f32], tolerance: f64, expected: impl Fn(usize) -> f64) {
    for frame in 0..frames(out) {
        let want = expected(frame);
        for channel in 0..2 {
            let got = f64::from(sample(out, channel, frame));
            assert!(
                (got - want).abs() <= tolerance,
                "frame {frame}, channel {channel}: {got} instead of {want}"
            );
        }
    }
}

// --- Frame exacto de cada golpe -----------------------------------------------------------

#[test]
fn rf_211_ca1_hits_start_on_their_exact_frame() {
    let hit_steps = [0, 4, 8, 12];
    for tempo in [120, 97] {
        let hits: Vec<_> = hit_steps.iter().map(|&s| (0, s, 2)).collect();
        let snapshot = snapshot()
            .tempo(tempo)
            .rhythm(rhythm_track("t1").pattern("p1", 16, &hits))
            .build();
        let pattern_end = step_frame(16, tempo);
        // Two loops of the pattern and the first hit of the third.
        let mut expected: Vec<usize> = Vec::new();
        for loop_start in [0, pattern_end] {
            expected.extend(
                hit_steps
                    .iter()
                    .map(|&s| loop_start + step_frame(s as u64, tempo)),
            );
        }
        expected.push(2 * pattern_end);
        if tempo == 120 {
            assert_eq!(expected[..4], [0, 24_000, 48_000, 72_000]);
        }

        for block in [480, 1, 64, 4_096, 5_000] {
            let mut h = Harness::new(&snapshot);
            h.play_pattern("t1", "p1");
            let out = h.run(2 * pattern_end + 1_000, block);
            let found = nonzero_frames(&out);
            let found_frames: Vec<usize> = found.iter().map(|(frame, _)| *frame).collect();
            assert_eq!(
                found_frames, expected,
                "at {tempo} in blocks of {block}: not a frame before or after"
            );
            for (frame, value) in found {
                assert_eq!(value, [NORMAL, NORMAL], "frame {frame} at {tempo}");
            }
        }
    }
}

#[test]
fn the_end_of_the_pattern_takes_the_position_back_to_0_and_step_0_sounds_again() {
    // 2 bars of 3/4 at 133: the end of the pattern is not a whole number of milliseconds.
    let tempo = 133;
    let snapshot = snapshot()
        .tempo(tempo)
        .time_signature("3/4")
        .rhythm(rhythm_track("t1").pattern("p1", 24, &[(0, 0, 2), (1, 23, 2)]))
        .build();
    let end = step_frame(24, tempo);
    assert_eq!(end, 129_925);

    let mut h = Harness::new(&snapshot);
    h.play_pattern("t1", "p1");
    let out = h.run(3 * end + 10, 480);
    let found: Vec<usize> = nonzero_frames(&out).iter().map(|(f, _)| *f).collect();
    let last_step = step_frame(23, tempo);
    assert_eq!(
        found,
        [
            0,
            last_step,
            end,
            end + last_step,
            2 * end,
            2 * end + last_step,
            3 * end
        ]
    );

    // The position counts from the start of the pattern again.
    let mut h = Harness::new(&snapshot);
    h.play_pattern("t1", "p1");
    h.run(end - 1, 1_000);
    assert_eq!(h.telemetry().position_frame, end as u64 - 1);
    h.run(1, 1_000);
    assert_eq!(h.telemetry().position_frame, 0);
    h.run(1_234, 1_000);
    assert_eq!(h.telemetry().position_frame, 1_234);
}

// --- Volumen y silencio de fila -----------------------------------------------------------

#[test]
fn a_hit_sounds_with_its_level_gain_times_the_row_gain_and_a_muted_row_is_silent() {
    // (row, level, row gain in dB): row r on step r.
    let rows: [(usize, i64, f64); 6] = [
        (0, 2, 0.0),
        (1, 1, 0.0),
        (2, 3, -6.0),
        (3, 1, 6.0),
        (5, 2, -48.0),
        (6, 1, -12.5),
    ];
    let mut track = rhythm_track("t1");
    let mut hits: Vec<_> = rows
        .iter()
        .map(|&(row, level, _)| (row, row, level))
        .collect();
    for &(row, _, gain_db) in &rows {
        track = track.row_gain_db(row, gain_db);
    }
    // Row 4 is muted and row 7 too, with the highest level: neither sounds.
    hits.push((4, 4, 2));
    hits.push((7, 7, 3));
    track = track.row_mute(4).row_mute(7).pattern("p1", 16, &hits);
    let mut h = Harness::new(&snapshot().rhythm(track).build());
    h.play_pattern("t1", "p1");
    let out = h.run(BAR_AT_120, 480);

    let found = nonzero_frames(&out);
    let found_frames: Vec<usize> = found.iter().map(|(f, _)| *f).collect();
    let expected_frames: Vec<usize> = rows.iter().map(|&(r, _, _)| r * STEP_AT_120).collect();
    assert_eq!(found_frames, expected_frames);
    for ((_, value), &(row, level, gain_db)) in found.iter().zip(&rows) {
        let expected = f64::from(LEVEL_GAINS[level as usize]) * db_gain(gain_db);
        for channel in value {
            assert!(
                (f64::from(*channel) - expected).abs() <= expected * 1e-6,
                "row {row}: {channel} instead of {expected}"
            );
        }
    }
}

#[test]
fn the_channel_applies_its_volume_and_pan_to_the_hits() {
    // (volume in dB, pan, expected left gain, expected right gain) (06 §10.2).
    let cases = [
        (-6.0, 0.0, db_gain(-6.0), db_gain(-6.0)),
        (0.0, 100.0, 0.0, 1.0),
        (0.0, -50.0, 1.0, 0.5),
        (-60.0, 0.0, 0.0, 0.0),
    ];
    for (gain_db, pan, left, right) in cases {
        let track = rhythm_track("t1")
            .gain_db(gain_db)
            .pan(pan)
            .pattern("p1", 16, &[(0, 0, 2)]);
        let mut h = Harness::new(&snapshot().rhythm(track).build());
        h.play_pattern("t1", "p1");
        let out = h.run(1_000, 480);
        for (channel, gain) in [(0, left), (1, right)] {
            let expected = f64::from(NORMAL) * gain;
            let got = f64::from(sample(&out, channel, 0));
            assert!(
                (got - expected).abs() <= 1e-6,
                "{gain_db} dB, pan {pan}, channel {channel}: {got} instead of {expected}"
            );
        }
        assert_eq!(first_nonzero_frame(&out, 1), None);
    }
}

// --- Grupo de corte ------------------------------------------------------------------------

#[test]
fn rf_213_ca1_a_closed_hat_one_step_after_an_open_hat_silences_it() {
    // Long sounds: the open hi-hat is 0,8 and the closed one 0,4, for 2 seconds.
    let kit = TestKit::impulses()
        .hat_choke()
        .sound(HAT_OPEN, constant(0.8, 96_000))
        .sound(HAT_CLOSED, constant(0.4, 96_000));
    let track = rhythm_track("t1").pattern("p1", 16, &[(HAT_OPEN, 0, 2), (HAT_CLOSED, 1, 2)]);
    let snapshot = snapshot().rhythm(track).build();
    let closed = STEP_AT_120;

    for block in [480, 4_096, 7] {
        let mut h = Harness::with_kit(&snapshot, &kit);
        h.play_pattern("t1", "p1");
        let out = h.run(20_000, block);
        // The open hi-hat falls to 0 in 240 frames from the frame where the closed one starts.
        assert_output(&out, 1e-6, |frame| {
            let open = if frame < closed {
                1.0
            } else if frame < closed + FADE {
                (FADE - (frame - closed)) as f64 / FADE as f64
            } else {
                0.0
            };
            let closed = if frame < closed { 0.0 } else { 0.4 };
            f64::from(NORMAL) * (0.8 * open + closed)
        });
    }
}

#[test]
fn a_hit_of_the_same_row_does_not_cut_the_previous_one() {
    let kit = TestKit::impulses()
        .hat_choke()
        .sound(HAT_OPEN, constant(0.25, 96_000));
    let track = rhythm_track("t1").pattern("p1", 16, &[(HAT_OPEN, 0, 2), (HAT_OPEN, 1, 2)]);
    let mut h = Harness::with_kit(&snapshot().rhythm(track).build(), &kit);
    h.play_pattern("t1", "p1");
    let out = h.run(30_000, 480);
    assert_output(&out, 1e-7, |frame| {
        let voices = if frame < STEP_AT_120 { 1.0 } else { 2.0 };
        voices * f64::from(NORMAL) * 0.25
    });
}

// --- Límite de voces -------------------------------------------------------------------------

#[test]
fn beyond_max_voices_the_oldest_voice_fades_out() {
    // Sounds of 2 seconds whose sample k is (k + 1) / 96 000, so that the output tells which
    // voices are sounding: each one contributes according to its age.
    let sound_frames = 96_000;
    let ramp: Vec<f32> = (0..sound_frames)
        .map(|k| (k + 1) as f32 / sound_frames as f32)
        .collect();
    let kit = TestKit::impulses().all_sounds(ramp.clone());
    // Hits on the 8 rows during 6 steps in a row: 48 hits.
    let steps = 6;
    let hits: Vec<_> = (0..steps)
        .flat_map(|step| (0..KIT_ROWS).map(move |row| (row, step, 2)))
        .collect();
    let mut track = rhythm_track("t1").pattern("p1", 16, &hits);
    let row_gain_db = -36.0;
    for row in 0..KIT_ROWS {
        track = track.row_gain_db(row, row_gain_db);
    }
    let mut h = Harness::with_kit(&snapshot().rhythm(track).build(), &kit);
    h.play_pattern("t1", "p1");
    let out = h.run(steps * STEP_AT_120 + 10_000, 480);

    // MAX_VOICES voices are 4 steps of 8 hits. From the fifth step on, each hit fades out the
    // oldest voice: on step s, the 8 voices of step s − 4.
    assert_eq!(MAX_VOICES, 4 * KIT_ROWS);
    let gain = f64::from(NORMAL) * db_gain(row_gain_db);
    assert_output(&out, 1e-5, |frame| {
        let mut sum = 0.0;
        for step in 0..steps {
            let start = step * STEP_AT_120;
            if frame < start {
                continue;
            }
            let fade_start = (step + 4) * STEP_AT_120;
            let factor = if step + 4 < steps && frame >= fade_start {
                let i = frame - fade_start;
                if i < FADE {
                    (FADE - i) as f64 / FADE as f64
                } else {
                    0.0
                }
            } else {
                1.0
            };
            sum += KIT_ROWS as f64 * gain * f64::from(ramp[frame - start]) * factor;
        }
        sum
    });
}

// --- Escucha -------------------------------------------------------------------------------

#[test]
fn rf_206_ca1_audition_sounds_from_the_first_frame_of_the_next_block() {
    // The track is muted and so is the row: the sound is heard all the same (06 §8.3).
    let track = rhythm_track("t1")
        .mute()
        .row_gain_db(2, -6.0)
        .row_mute(2)
        .pattern("p1", 16, &[(0, 0, 2)]);
    let snapshot = snapshot().rhythm(track).build();
    let mut h = Harness::new(&snapshot);
    let before = h.run(1_000, 480);
    assert_eq!(first_nonzero_frame(&before, 0), None);

    h.audition("t1", 2);
    let out = h.run(480, 480);
    let expected = f64::from(NORMAL) * db_gain(-6.0);
    assert_eq!(nonzero_frames(&out).len(), 1);
    for channel in 0..2 {
        let got = f64::from(sample(&out, channel, 0));
        assert!((got - expected).abs() <= expected * 1e-6, "{got}");
    }

    // The audition changes nothing else: the transport is still stopped, and playing afterwards
    // sounds exactly as if there had been no audition.
    let t = h.telemetry();
    assert_eq!(
        (t.state, t.mode, t.position_frame),
        (TelemetryState::Stopped, None, 0)
    );
    h.play_pattern("t1", "p1");
    let after = h.run(BAR_AT_120, 480);
    let mut fresh = Harness::new(&snapshot);
    fresh.play_pattern("t1", "p1");
    assert_eq!(after, fresh.run(BAR_AT_120, 480));
}

#[test]
fn audition_while_playing_sounds_through_the_track_that_plays() {
    let snapshot = snapshot()
        .rhythm(rhythm_track("t1").pattern("p1", 16, &[(0, 0, 2)]))
        .rhythm(rhythm_track("t2").pattern("p2", 16, &[]))
        .build();
    let mut h = Harness::new(&snapshot);
    h.play_pattern("t1", "p1");
    h.run(1_000, 500);
    h.audition("t1", 1);
    let out = h.run(500, 500);
    assert_eq!(nonzero_frames(&out), [(0, [NORMAL, NORMAL])]);

    // In pattern mode only the chosen track is audible (06 §7, §8.3).
    h.audition("t2", 1);
    let out = h.run(500, 500);
    assert_eq!(first_nonzero_frame(&out, 0), None);
}

// --- Modo patrón ------------------------------------------------------------------------------

#[test]
fn pattern_mode_plays_only_the_chosen_track_even_if_it_is_muted() {
    let snapshot = snapshot()
        .rhythm(rhythm_track("t1").pattern("p1", 16, &[(0, 0, 2)]))
        .rhythm(
            rhythm_track("t2")
                .mute()
                .pattern("p2", 16, &[(0, 1, 2)])
                .pattern("p3", 16, &[(0, 2, 2)]),
        )
        .rhythm(rhythm_track("t3").solo().pattern("p4", 16, &[(0, 3, 2)]))
        .build();
    let mut h = Harness::new(&snapshot);
    h.play_pattern("t2", "p3");
    let out = h.run(BAR_AT_120, 480);
    assert_eq!(nonzero_frames(&out), [(2 * STEP_AT_120, [NORMAL, NORMAL])]);
}

// --- Parar -----------------------------------------------------------------------------------

#[test]
fn rf_101_ca2_stop_fades_out_without_clicks() {
    // A 440 Hz sine of amplitude 0,5 for 4 seconds.
    let sine: Vec<f32> = (0..192_000)
        .map(|i| (0.5 * (2.0 * PI * 440.0 * f64::from(i) / 48_000.0).sin()) as f32)
        .collect();
    let kit = TestKit::impulses().sound(0, sine);
    let snapshot = snapshot()
        .rhythm(rhythm_track("t1").pattern("p1", 16, &[(0, 0, 2)]))
        .build();
    let stop_at = 30_000;
    let after = 2_000;

    let mut reference = Harness::with_kit(&snapshot, &kit);
    reference.play_pattern("t1", "p1");
    let reference = reference.run(stop_at + after, 480);

    let mut h = Harness::with_kit(&snapshot, &kit);
    h.play_pattern("t1", "p1");
    let mut out = h.run(stop_at, 480);
    h.stop();
    out.extend(h.run(after, 480));

    // The same as without stopping until the stop, then multiplied by 1 − (i + 1)/240.
    assert_eq!(out[..2 * stop_at], reference[..2 * stop_at]);
    for i in 0..FADE {
        let frame = stop_at + i;
        let ramp = 1.0 - (i + 1) as f64 / FADE as f64;
        for channel in 0..2 {
            let expected = f64::from(sample(&reference, channel, frame)) * ramp;
            let got = f64::from(sample(&out, channel, frame));
            assert!((got - expected).abs() <= 1e-6, "frame {frame}: {got}");
        }
    }
    // No jump larger than that of the signal itself (plus the ramp's own step), then exact
    // silence although the sound has not ended.
    let range = stop_at - 1..stop_at + FADE + 1;
    assert!(max_step(&out, range.clone()) <= max_step(&reference, range) + 0.5 / FADE as f32);
    assert_eq!(sample(&out, 0, stop_at + FADE - 1), 0.0);
    assert_eq!(first_nonzero_frame(&out, stop_at + FADE - 1), None);
    assert_eq!(h.telemetry().state, TelemetryState::Stopped);

    // Nothing sounds afterwards either: the voices were switched off.
    assert_eq!(first_nonzero_frame(&h.run(10_000, 480), 0), None);
}

#[test]
fn play_during_the_stop_fade_ends_it_and_plays_from_the_start() {
    let kit = TestKit::impulses().sound(1, constant(0.5, 96_000));
    let snapshot = snapshot()
        .rhythm(rhythm_track("t1").pattern("p1", 16, &[(1, 0, 2), (0, 4, 2)]))
        .build();
    let mut h = Harness::with_kit(&snapshot, &kit);
    h.play_pattern("t1", "p1");
    h.run(10_000, 500);
    h.stop();
    h.run(100, 100);
    h.play_pattern("t1", "p1");
    let out = h.run(4 * STEP_AT_120 + 1, 500);

    // The long sound of the previous playback has gone; the new one starts on frame 0.
    let mut fresh = Harness::with_kit(&snapshot, &kit);
    fresh.play_pattern("t1", "p1");
    assert_eq!(out, fresh.run(4 * STEP_AT_120 + 1, 500));
}

#[test]
fn stop_when_stopped_is_ignored() {
    let mut h = Harness::new(&snapshot().rhythm(rhythm_track("t1")).build());
    h.stop();
    h.audition("t1", 0);
    let out = h.run(480, 480);
    assert_eq!(nonzero_frames(&out), [(0, [NORMAL, NORMAL])]);
}

// --- Telemetría -------------------------------------------------------------------------------

#[test]
fn telemetry_gives_the_state_mode_position_and_step_of_pattern_mode() {
    let tempo = 97;
    let snapshot = snapshot()
        .tempo(tempo)
        .rhythm(rhythm_track("t1").pattern("p1", 32, &[]))
        .build();
    let mut h = Harness::new(&snapshot);
    let t = h.telemetry();
    assert_eq!(
        (t.state, t.mode, t.pattern_step),
        (TelemetryState::Stopped, None, None)
    );

    h.play_pattern("t1", "p1");
    let end = step_frame(32, tempo) as u64;
    for (run, position) in [(10_000, 10_000), (100_000, 110_000), (end - 110_000 + 5, 5)] {
        h.run(run as usize, 480);
        let t = h.telemetry();
        assert_eq!(t.state, TelemetryState::Playing);
        assert_eq!(t.mode, Some(TelemetryMode::Pattern));
        assert_eq!(t.position_frame, position);
        // floor(position · tempoBpm / 720 000) (06 §18.1).
        let step = (position * u64::from(tempo) / 720_000) as u32;
        assert_eq!(t.pattern_step, Some(step));
    }
    assert_eq!(h.telemetry().pattern_step, Some(0));
    assert!(h.telemetry().track_peaks.contains_key("t1"));

    // While stopping and once stopped, it is reported as stopped (06 §6.1).
    h.stop();
    h.run(100, 100);
    let t = h.telemetry();
    assert_eq!(
        (t.state, t.mode, t.position_frame, t.pattern_step),
        (TelemetryState::Stopped, None, 0, None)
    );
    h.run(1_000, 100);
    assert_eq!(h.telemetry().state, TelemetryState::Stopped);
}

// --- EngineHandle -------------------------------------------------------------------------

#[test]
fn engine_handle_validates_and_accepts_snapshots_and_plays_patterns() {
    let engine = testing::start_engine_without_output(EngineConfig {
        kits_dir: kits_dir(),
    })
    .expect("the engine starts without an output device");
    let handle = engine.handle();

    let mut invalid = snapshot().build();
    invalid.tempo_bpm = 300.0;
    assert_eq!(
        handle.set_project(invalid).map_err(|e| e.code()),
        Err("INVALID_SNAPSHOT")
    );

    let valid = snapshot()
        .rhythm(rhythm_track("t1").pattern("p1", 16, &[(0, 0, 2)]))
        .rhythm(rhythm_track("t2"))
        .build();
    assert_eq!(handle.set_project(valid), Ok(()));
    let peaks = handle.telemetry().track_peaks;
    assert_eq!(peaks.keys().collect::<Vec<_>>(), ["t1", "t2"]);

    // Without an output stream, pattern mode cannot play; audition and stop do nothing.
    let pattern = PlayRequest::Pattern {
        track_id: "t1".to_string(),
        pattern_id: "p1".to_string(),
    };
    assert_eq!(
        handle.play(pattern).map_err(|e| e.code()),
        Err("NO_OUTPUT_DEVICE")
    );
    handle.audition("t1", 0);
    handle.stop();
    assert_eq!(handle.telemetry().state, TelemetryState::Stopped);
    assert!(handle.poll_events().is_empty());
}

#[test]
fn engine_start_fails_without_the_kits() {
    let missing = tempfile::tempdir().expect("a temporary folder");
    let result = testing::start_engine_without_output(EngineConfig {
        kits_dir: missing.path().join("kits"),
    });
    assert!(matches!(result, Err(EngineError::NotFound(_))));
}
