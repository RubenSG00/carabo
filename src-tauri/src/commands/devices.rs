//! The device commands (04 §6.2).
// TODO(T-086): `devices_list`, `devices_select` and `calibration_run`.

use tauri::State;

use crate::error::BridgeError;
use crate::state::AppState;

#[tauri::command]
pub async fn engine_test_tone(state: State<'_, AppState>) -> Result<(), BridgeError> {
    Ok(state.engine().test_tone()?)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use carabo_engine::testing::start_engine_without_output;
    use carabo_engine::EngineConfig;

    use crate::error::{BridgeError, BridgeErrorCode};

    #[test]
    fn without_an_output_device_the_test_tone_gives_no_output_device() {
        let kits_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("assets")
            .join("kits");
        let engine =
            start_engine_without_output(EngineConfig { kits_dir }).expect("the engine starts");
        let error = BridgeError::from(engine.handle().test_tone().expect_err("no output"));
        assert_eq!(error.code, BridgeErrorCode::NoOutputDevice);
    }
}
