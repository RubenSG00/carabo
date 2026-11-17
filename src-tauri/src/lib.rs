#![forbid(unsafe_code)]

pub mod commands;
pub mod error;
pub mod logging;
pub mod paths;
pub mod state;
pub mod telemetry;

use carabo_engine::{Engine, EngineConfig, EngineError};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::logging::{Level, Log};
use crate::paths::AppPaths;
use crate::state::AppState;

/// Module name of the lines that the start-up writes to the log (05 §7.3).
const MODULE: &str = "app";
/// The only visible text that does not go through the translation files: the interface does
/// not exist yet when it is shown (07 §6.1).
const ENGINE_START_FAILED: &str = "Carabo could not start its audio engine. See the log file.";

/// Starts the native program (07 §6.1): registers the plugins and, in `setup`, computes the
/// paths, opens the log, starts the engine, keeps the `AppState` and launches the telemetry
/// thread.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let paths = AppPaths::resolve(app.handle())?;
            // Without a log file the program works all the same; it only loses the log.
            logging::install(Log::open(&paths.logs_dir()).unwrap_or_else(|_| Log::disabled()));
            let config = EngineConfig {
                kits_dir: paths.kits_dir.clone(),
            };
            match Engine::start(config) {
                Ok(engine) => {
                    let handle = engine.handle();
                    app.manage(AppState::new(engine, paths));
                    telemetry::spawn(app.handle().clone(), handle)?;
                }
                Err(error) => report_engine_failure(app.handle(), &error),
            }
            Ok(())
        })
        // The commands of the tasks done so far (04 §6.2). TODO(T-050, T-051, T-071, T-086,
        // T-101): the rest of the commands, each with its task.
        .invoke_handler(tauri::generate_handler![
            commands::engine::engine_set_project,
            commands::engine::engine_play,
            commands::engine::engine_stop,
            commands::engine::engine_seek,
            commands::engine::engine_set_loop,
            commands::engine::engine_set_metronome,
            commands::engine::engine_audition,
            commands::engine::engine_input_open,
            commands::engine::engine_input_close,
            commands::devices::engine_test_tone,
            commands::system::app_info,
            commands::system::log_write,
        ])
        .run(tauri::generate_context!())
        // `run` only fails when the window system cannot start, and then there is no window in
        // which to report the error. TODO(T-051): write the error to the log before exiting.
        .expect("Carabo could not start its window");
}

/// The engine has not started (07 §6.1): writes the error to the log and ends the program with a
/// native error dialog that gives the path of the log. The dialog does not block: `setup` runs
/// on the main thread, which must keep running for the dialog to show.
fn report_engine_failure(app: &AppHandle, error: &EngineError) {
    let log = logging::log();
    log.write(
        Level::Error,
        MODULE,
        &format!("the audio engine did not start: {error}"),
    );
    let text = match log.path() {
        Some(path) => format!("{ENGINE_START_FAILED}\n{}", path.display()),
        None => ENGINE_START_FAILED.to_string(),
    };
    let exit = app.clone();
    app.dialog()
        .message(text)
        .kind(MessageDialogKind::Error)
        .show(move |_| exit.exit(1));
}
