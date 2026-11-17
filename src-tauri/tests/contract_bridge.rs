//! Contract of the bridge types (08 §7): each Rust type, built with the values of its file of
//! `fixtures/bridge/`, serializes to a JSON equal to that file.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use carabo_engine::{Telemetry, TelemetryMode, TelemetryState};
use carabo_lib::error::{BridgeError, BridgeErrorCode};
use serde::Serialize;
use serde_json::Value;

/// The JSON of `fixtures/bridge/<name>`.
fn fixture(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixtures")
        .join("bridge")
        .join(name);
    let text = fs::read_to_string(&path).unwrap_or_else(|error| panic!("{path:?}: {error}"));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{path:?}: {error}"))
}

fn json_of(value: &impl Serialize) -> Value {
    serde_json::to_value(value).expect("a bridge type serializes")
}

#[test]
fn telemetry_serializes_like_its_fixture() {
    let telemetry = Telemetry {
        state: TelemetryState::Playing,
        mode: Some(TelemetryMode::Pattern),
        position_frame: 12_000,
        pattern_step: Some(1),
        track_peaks: BTreeMap::from([("track-1".to_string(), [0.5, 0.25])]),
        master_peak: [0.5, 0.25],
        input_peak: None,
        xruns: 0,
        recorded_frames: None,
        output_available: true,
    };
    assert_eq!(json_of(&telemetry), fixture("telemetry.json"));
}

#[test]
fn bridge_error_serializes_like_its_fixture() {
    let error = BridgeError::new(BridgeErrorCode::NoOutputDevice, "there is no output stream");
    assert_eq!(json_of(&error), fixture("bridge-error.json"));
}
