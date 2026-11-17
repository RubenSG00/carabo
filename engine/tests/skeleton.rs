//! The engine skeleton (T-006): the test tone, the engine clock, starting without an output
//! device and the methods of `EngineHandle` that later tasks implement.

mod common;

use std::f64::consts::PI;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use carabo_engine::testing::{self, RtCommand};
use carabo_engine::{
    Engine, EngineConfig, EngineError, PlayRequest, RecordRequest, TelemetryState,
};
use common::{first_nonzero_frame, frames, peak, sample, snapshot, tone_level, Harness};

/// Frames of the test tone (06 §12).
const TONE_FRAMES: usize = 48_000;
/// Frames of each fade of the test tone (06 §12).
const FADE_FRAMES: usize = 240;

fn kits_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/kits")
}

/// The test tone of 06 §12 at frame `i` of the tone: a 440 Hz sine of amplitude 0,25 with a
/// linear fade of 240 frames at each end, so that its first and last samples are 0.
fn expected_tone(i: usize) -> f64 {
    let fade_in = i as f64 / FADE_FRAMES as f64;
    let fade_out = (TONE_FRAMES - 1 - i) as f64 / FADE_FRAMES as f64;
    let gain = fade_in.min(fade_out).min(1.0);
    0.25 * (2.0 * PI * 440.0 * i as f64 / 48_000.0).sin() * gain
}

fn assert_follows_the_tone(out: &[f32], tone_start: usize) {
    for i in 0..TONE_FRAMES {
        let frame = tone_start + i;
        let left = sample(out, 0, frame);
        assert_eq!(
            left,
            sample(out, 1, frame),
            "frame {frame}: both channels are equal"
        );
        assert!(
            (f64::from(left) - expected_tone(i)).abs() < 1e-6,
            "frame {frame}: {left} instead of {}",
            expected_tone(i)
        );
    }
}

#[test]
fn test_tone_is_440_hz_at_a_quarter_with_240_frame_fades_then_exact_silence() {
    let mut h = Harness::new(&snapshot().build());
    h.send(RtCommand::TestTone);
    let out = h.run(TONE_FRAMES + 9_600, 480);

    // A whole number of periods (11 periods are 1 200 frames) between the fades.
    for channel in 0..2 {
        let level = tone_level(&out, 440.0, channel, 1_200..46_800);
        assert!((level - 0.25).abs() <= 0.01, "channel {channel}: {level}");
    }
    assert_follows_the_tone(&out, 0);

    // The fades: 0 at both ends, full level 240 frames inside.
    assert_eq!(first_nonzero_frame(&out, 0), Some(1));
    assert_eq!(sample(&out, 0, TONE_FRAMES - 1), 0.0);
    assert!(peak(&out, 0, 0..FADE_FRAMES / 4) < 0.25 / 4.0);
    assert!(peak(&out, 0, FADE_FRAMES..1_200) > 0.249);

    // After the tone, exact silence.
    assert_eq!(first_nonzero_frame(&out, TONE_FRAMES), None);
    assert_eq!(frames(&out), TONE_FRAMES + 9_600);
}

#[test]
fn test_tone_does_not_depend_on_the_block_size() {
    let mut reference = Harness::new(&snapshot().build());
    reference.send(RtCommand::TestTone);
    let expected = reference.run(TONE_FRAMES + 1_000, 480);
    for block in [1, 64, 1_024, 4_096, 6_000] {
        let mut h = Harness::new(&snapshot().build());
        h.send(RtCommand::TestTone);
        assert_eq!(
            h.run(TONE_FRAMES + 1_000, block),
            expected,
            "block of {block}"
        );
    }
}

#[test]
fn a_test_tone_that_arrives_while_one_sounds_is_ignored() {
    let mut h = Harness::new(&snapshot().build());
    h.send(RtCommand::TestTone);
    let mut out = h.run(10_000, 500);
    h.send(RtCommand::TestTone);
    out.extend(h.run(TONE_FRAMES, 500));

    assert_follows_the_tone(&out, 0);
    assert_eq!(first_nonzero_frame(&out, TONE_FRAMES), None);
}

#[test]
fn a_test_tone_can_sound_again_after_the_previous_one_ends() {
    let mut h = Harness::new(&snapshot().build());
    h.send(RtCommand::TestTone);
    h.run(TONE_FRAMES + 100, 480);
    h.send(RtCommand::TestTone);
    let out = h.run(TONE_FRAMES + 100, 480);

    assert_follows_the_tone(&out, 0);
    assert_eq!(first_nonzero_frame(&out, TONE_FRAMES), None);
}

#[test]
fn without_commands_the_output_is_exact_silence() {
    let mut h = Harness::new(&snapshot().build());
    let out = h.run(10_000, 480);
    assert_eq!(first_nonzero_frame(&out, 0), None);
    assert_eq!(h.clipped_samples(), 0);
}

#[test]
fn engine_clock_advances_exactly_the_frames_computed() {
    for block in [1, 63, 480, 4_096, 5_000] {
        let mut h = Harness::new(&snapshot().build());
        assert_eq!(h.engine_clock(), 0);
        h.run(12_345, block);
        assert_eq!(h.engine_clock(), 12_345, "blocks of {block}");
        h.run(0, block);
        assert_eq!(h.engine_clock(), 12_345, "blocks of {block}");
        h.run(10_000, block);
        assert_eq!(h.engine_clock(), 22_345, "blocks of {block}");
    }
}

#[test]
fn engine_clock_advances_while_the_test_tone_sounds() {
    let mut h = Harness::new(&snapshot().build());
    h.send(RtCommand::TestTone);
    h.run(30_000, 480);
    assert_eq!(h.engine_clock(), 30_000);
}

#[test]
fn telemetry_of_a_stopped_processor() {
    let mut h = Harness::new(&snapshot().build());
    h.send(RtCommand::TestTone);
    h.run(4_800, 480);
    let t = h.telemetry();
    assert_eq!(t.state, TelemetryState::Stopped);
    assert_eq!(t.mode, None);
    assert_eq!(t.position_frame, 0);
    assert_eq!(t.pattern_step, None);
    assert!(t.track_peaks.is_empty());
    assert_eq!(t.input_peak, None);
    assert_eq!(t.xruns, 0);
    assert_eq!(t.recorded_frames, None);
}

#[test]
fn engine_starts_without_an_output_device() {
    let engine = testing::start_engine_without_output(EngineConfig {
        kits_dir: kits_dir(),
    })
    .expect("the engine starts without an output device");
    let handle = engine.handle();

    let telemetry = handle.telemetry();
    assert!(!telemetry.output_available);
    assert_eq!(telemetry.state, TelemetryState::Stopped);
    assert_eq!(telemetry.mode, None);

    let error = handle.test_tone().expect_err("there is no output stream");
    assert_eq!(error.code(), "NO_OUTPUT_DEVICE");
    assert!(handle.poll_events().is_empty());
}

#[test]
fn handle_methods_of_later_tasks_are_not_implemented_yet() {
    let engine = testing::start_engine_without_output(EngineConfig {
        kits_dir: kits_dir(),
    })
    .expect("the engine starts without an output device");
    let handle = engine.handle();

    // 09 §1.3, rule 7: those with a result fail with ENGINE_INTERNAL "not implemented".
    let not_implemented = |result: Result<(), EngineError>| {
        assert_eq!(
            result,
            Err(EngineError::EngineInternal("not implemented".to_string()))
        );
    };
    not_implemented(handle.play(PlayRequest::Song { start_frame: 0 }));
    not_implemented(
        handle
            .record_start(RecordRequest {
                track_id: "t1".to_string(),
                start_frame: 0,
                project_dir: PathBuf::from("project"),
            })
            .map(|_| ()),
    );
    not_implemented(handle.input_open());
    not_implemented(handle.select_devices(None, None).map(|_| ()));
    not_implemented(handle.calibrate().map(|_| ()));

    // Those without a result do nothing.
    handle.seek(48_000);
    handle.set_loop(true, 0, 96_000);
    handle.set_metronome(true);
    handle.input_close();
    let devices = handle.devices();
    assert!(devices.outputs.is_empty() && devices.inputs.is_empty());
    let _pool = handle.audio_pool();
    assert!(handle.poll_events().is_empty());
    assert_eq!(handle.telemetry().state, TelemetryState::Stopped);
}

#[test]
fn a_handle_that_outlives_its_engine_returns_an_error() {
    let engine = testing::start_engine_without_output(EngineConfig {
        kits_dir: kits_dir(),
    })
    .expect("the engine starts without an output device");
    let handle = engine.handle();
    drop(engine);
    assert_eq!(
        handle.test_tone().map_err(|e| e.code()),
        Err("ENGINE_INTERNAL")
    );
}

/// Needs a real output device: run it with `cargo test -p carabo-engine -- --ignored device_`
/// and listen for one second of a 440 Hz tone (PA-05).
#[test]
#[ignore]
fn device_output_plays_test_tone() {
    let engine = Engine::start(EngineConfig {
        kits_dir: kits_dir(),
    })
    .expect("the engine starts");
    let handle = engine.handle();
    assert!(
        handle.telemetry().output_available,
        "there is an output device"
    );

    let clock_before = testing::engine_clock(&handle);
    handle.test_tone().expect("the test tone starts");
    thread::sleep(Duration::from_millis(1_500));

    // The output thread has computed at least the whole tone, without internal errors.
    assert!(testing::engine_clock(&handle) >= clock_before + TONE_FRAMES as u64);
    assert!(handle.poll_events().is_empty());
}
