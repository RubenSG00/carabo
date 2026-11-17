//! The state shared by the commands (04 §4), which they receive with
//! `tauri::State<'_, AppState>` (07 §6.2).

use carabo_engine::{Engine, EngineHandle};

use crate::paths::AppPaths;

/// What the native program keeps while it runs.
// TODO(T-101): the cancellation flag of the export in progress.
pub struct AppState {
    /// The engine owns its threads: it lives as long as the application.
    _engine: Engine,
    handle: EngineHandle,
    paths: AppPaths,
}

impl AppState {
    /// The state of a running engine.
    pub fn new(engine: Engine, paths: AppPaths) -> Self {
        let handle = engine.handle();
        Self {
            _engine: engine,
            handle,
            paths,
        }
    }

    /// The handle to talk to the engine.
    pub fn engine(&self) -> &EngineHandle {
        &self.handle
    }

    /// The folders of the application.
    pub fn paths(&self) -> &AppPaths {
        &self.paths
    }
}
