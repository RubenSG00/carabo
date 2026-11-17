#![forbid(unsafe_code)]

/// Starts the native program: registers the plugins and opens the main window (07 §6.1).
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        // TODO(T-008): in `setup`, start the engine, manage the `AppState` and launch the
        // telemetry thread; register the bridge commands with `tauri::generate_handler!`.
        // TODO(T-051): in `setup`, compute the paths and open the log.
        .run(tauri::generate_context!())
        // `run` only fails when the window system cannot start, and then there is no window in
        // which to report the error. TODO(T-051): write the error to the log before exiting.
        .expect("Carabo could not start its window");
}
