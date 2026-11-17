//! The system commands (04 §6.2): `app_info` and `log_write`.

use serde::Serialize;
use tauri::AppHandle;

use crate::error::BridgeError;
use crate::logging::{self, Level, Log};

/// Module name of the lines that the interface writes to the log (05 §7.3).
pub const INTERFACE_MODULE: &str = "ui";

/// Version of the program and language of Windows (04 §6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    /// For example `"es-ES"`; empty if Windows does not report one.
    pub system_locale: String,
}

/// The `AppInfo` of a program with this version, on a system with this language.
pub fn app_info_of(version: String, system_locale: Option<String>) -> AppInfo {
    AppInfo {
        version,
        system_locale: system_locale.unwrap_or_default(),
    }
}

/// Writes a line of the interface to `log`.
pub fn write_interface_line(log: &Log, level: Level, message: &str) {
    log.write(level, INTERFACE_MODULE, message);
}

/// The version is that of `package.json`, which Tauri takes through `tauri.conf.json`.
#[tauri::command]
pub async fn app_info(app: AppHandle) -> Result<AppInfo, BridgeError> {
    Ok(app_info_of(
        app.package_info().version.to_string(),
        sys_locale::get_locale(),
    ))
}

#[tauri::command]
pub async fn log_write(level: Level, message: String) -> Result<(), BridgeError> {
    write_interface_line(logging::log(), level, &message);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use serde_json::json;

    #[test]
    fn app_info_serializes_with_the_keys_of_04_6_1() {
        let info = app_info_of("0.1.0".to_string(), Some("es-ES".to_string()));
        assert_eq!(
            serde_json::to_value(info).expect("app info serializes"),
            json!({ "version": "0.1.0", "systemLocale": "es-ES" })
        );
    }

    #[test]
    fn without_a_language_of_windows_the_locale_is_empty() {
        assert_eq!(app_info_of("0.1.0".to_string(), None).system_locale, "");
    }

    #[test]
    fn the_interface_writes_its_lines_with_the_module_ui() {
        let data = tempfile::tempdir().expect("a temporary folder");
        let log = Log::open(data.path()).expect("the log opens");
        write_interface_line(&log, Level::Error, "unexpected exception");
        let text = fs::read_to_string(data.path().join("carabo.log")).expect("the log exists");
        assert!(
            text.ends_with("Z ERROR ui unexpected exception\n"),
            "{text}"
        );
    }
}
