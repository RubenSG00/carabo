//! The `engine_*` commands (04 §6.2).
// TODO(T-086): `engine_record_start`.

use carabo_engine::{EngineHandle, EngineSnapshot, PlayRequest};
use serde::Deserialize;
use serde_json::Value;
use tauri::State;

use crate::error::{BridgeError, BridgeErrorCode};
use crate::state::AppState;

/// State and range of the loop (04 §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopRequest {
    pub enabled: bool,
    pub start_frame: u64,
    pub end_frame: u64,
}

/// Hands a snapshot to the engine. Whatever does not have the shape of 05 §10.1, like what
/// breaks a rule of 06 §5.5, is `INVALID_SNAPSHOT`: the only error of `engine_set_project`
/// (04 §6.2).
pub fn set_project(engine: &EngineHandle, snapshot: Value) -> Result<(), BridgeError> {
    let snapshot: EngineSnapshot = serde_json::from_value(snapshot)
        .map_err(|error| BridgeError::new(BridgeErrorCode::InvalidSnapshot, error.to_string()))?;
    Ok(engine.set_project(snapshot)?)
}

#[tauri::command]
pub async fn engine_set_project(
    state: State<'_, AppState>,
    snapshot: Value,
) -> Result<(), BridgeError> {
    set_project(state.engine(), snapshot)
}

#[tauri::command]
pub async fn engine_play(
    state: State<'_, AppState>,
    request: PlayRequest,
) -> Result<(), BridgeError> {
    Ok(state.engine().play(request)?)
}

#[tauri::command]
pub async fn engine_stop(state: State<'_, AppState>) -> Result<(), BridgeError> {
    state.engine().stop();
    Ok(())
}

#[tauri::command]
pub async fn engine_seek(state: State<'_, AppState>, frame: u64) -> Result<(), BridgeError> {
    state.engine().seek(frame);
    Ok(())
}

/// The argument is called `loop` in the interface (`invoke("engine_set_loop", { loop })`).
#[tauri::command]
pub async fn engine_set_loop(
    state: State<'_, AppState>,
    r#loop: LoopRequest,
) -> Result<(), BridgeError> {
    state
        .engine()
        .set_loop(r#loop.enabled, r#loop.start_frame, r#loop.end_frame);
    Ok(())
}

#[tauri::command]
pub async fn engine_set_metronome(
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), BridgeError> {
    state.engine().set_metronome(enabled);
    Ok(())
}

#[tauri::command]
pub async fn engine_audition(
    state: State<'_, AppState>,
    track_id: String,
    row: usize,
) -> Result<(), BridgeError> {
    state.engine().audition(&track_id, row);
    Ok(())
}

#[tauri::command]
pub async fn engine_input_open(state: State<'_, AppState>) -> Result<(), BridgeError> {
    Ok(state.engine().input_open()?)
}

#[tauri::command]
pub async fn engine_input_close(state: State<'_, AppState>) -> Result<(), BridgeError> {
    state.engine().input_close();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use carabo_engine::testing::start_engine_without_output;
    use carabo_engine::{Engine, EngineConfig};
    use serde_json::json;

    fn engine() -> Engine {
        let kits_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("assets")
            .join("kits");
        start_engine_without_output(EngineConfig { kits_dir }).expect("the engine starts")
    }

    fn empty_snapshot() -> Value {
        json!({
            "snapshotVersion": 1,
            "tempoBpm": 120,
            "timeSignature": "4/4",
            "tracks": [],
            "master": { "gainDb": 0 }
        })
    }

    #[test]
    fn a_valid_snapshot_reaches_the_engine() {
        let engine = engine();
        assert_eq!(set_project(&engine.handle(), empty_snapshot()), Ok(()));
    }

    #[test]
    fn a_snapshot_that_breaks_a_rule_of_06_5_5_is_invalid_snapshot() {
        let engine = engine();
        let mut snapshot = empty_snapshot();
        snapshot["tempoBpm"] = json!(300);
        let error = set_project(&engine.handle(), snapshot).expect_err("300 is out of range");
        assert_eq!(error.code, BridgeErrorCode::InvalidSnapshot);
    }

    #[test]
    fn a_snapshot_without_the_shape_of_05_10_1_is_invalid_snapshot() {
        let engine = engine();
        for snapshot in [
            json!({}),
            json!("snapshot"),
            json!({ "snapshotVersion": 1 }),
        ] {
            let error = set_project(&engine.handle(), snapshot).expect_err("not a snapshot");
            assert_eq!(error.code, BridgeErrorCode::InvalidSnapshot);
        }
    }

    #[test]
    fn without_an_output_device_playing_gives_no_output_device() {
        let engine = engine();
        let handle = engine.handle();
        set_project(&handle, empty_snapshot()).expect("an empty snapshot is valid");
        let request: PlayRequest = serde_json::from_value(json!({
            "mode": "pattern",
            "trackId": "track-1",
            "patternId": "pattern-1"
        }))
        .expect("a play request deserializes from its JSON of 04 §6.1");
        let error = BridgeError::from(handle.play(request).expect_err("there is no output"));
        assert_eq!(error.code, BridgeErrorCode::NoOutputDevice);
    }

    #[test]
    fn a_loop_request_deserializes_from_its_json_of_04_6_1() {
        let request: LoopRequest = serde_json::from_value(json!({
            "enabled": true,
            "startFrame": 0,
            "endFrame": 384_000
        }))
        .expect("a loop request deserializes");
        assert_eq!(
            request,
            LoopRequest {
                enabled: true,
                start_frame: 0,
                end_frame: 384_000
            }
        );
    }
}
