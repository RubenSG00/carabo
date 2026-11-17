//! The folders of the application (05 §7, 07 §6.6) and the kits folder (07 §6.1). The
//! application folders come from Tauri, never from environment variables.

use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager, Runtime};

/// Name of the folder of the log inside the local data folder (05 §7).
const LOGS_DIR_NAME: &str = "logs";
/// Name of the kits folder next to the executable (07 §6.1).
const KITS_DIR_NAME: &str = "kits";

/// The folders the native program uses.
// TODO(T-050): the `Documents\Carabo` folder of `default_projects_dir`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppPaths {
    /// `%APPDATA%\com.carabo.app\`: settings and session (05 §7).
    pub config_dir: PathBuf,
    /// `%LOCALAPPDATA%\com.carabo.app\`: unsaved projects and the log (05 §7).
    pub local_data_dir: PathBuf,
    /// The folder with the three kits that the engine loads.
    pub kits_dir: PathBuf,
}

impl AppPaths {
    /// Asks Tauri for the folders of the application and finds the kits folder.
    pub fn resolve<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<AppPaths> {
        let path = app.path();
        let exe_dir = tauri::utils::platform::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(Path::to_path_buf));
        Ok(AppPaths {
            config_dir: path.app_config_dir()?,
            local_data_dir: path.app_local_data_dir()?,
            kits_dir: kits_dir(exe_dir.as_deref(), development_kits_dir().as_deref()),
        })
    }

    /// The folder of the log (05 §7.3).
    pub fn logs_dir(&self) -> PathBuf {
        self.local_data_dir.join(LOGS_DIR_NAME)
    }
}

/// The kits folder (07 §6.1): the `kits` folder next to the executable, if it exists;
/// otherwise, in development builds only, `development_dir`. If neither applies, the folder
/// next to the executable all the same: the engine then fails to start with `NOT_FOUND` and
/// the error says which folder is missing.
pub fn kits_dir(exe_dir: Option<&Path>, development_dir: Option<&Path>) -> PathBuf {
    let beside_exe = exe_dir.map_or_else(
        || PathBuf::from(KITS_DIR_NAME),
        |dir| dir.join(KITS_DIR_NAME),
    );
    match development_dir {
        Some(dir) if !beside_exe.is_dir() => dir.to_path_buf(),
        _ => beside_exe,
    }
}

/// `assets/kits` of the repository, fixed when compiling, in development builds only
/// (`cfg!(debug_assertions)`, 07 §6.1).
fn development_kits_dir() -> Option<PathBuf> {
    cfg!(debug_assertions).then(|| {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("assets")
            .join("kits")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn the_kits_folder_next_to_the_executable_comes_first() {
        let exe_dir = tempfile::tempdir().expect("a temporary folder");
        fs::create_dir(exe_dir.path().join("kits")).expect("the kits folder is created");
        let development = Path::new("/repository/assets/kits");
        assert_eq!(
            kits_dir(Some(exe_dir.path()), Some(development)),
            exe_dir.path().join("kits")
        );
    }

    #[test]
    fn without_kits_next_to_the_executable_a_development_build_uses_the_repository() {
        let exe_dir = tempfile::tempdir().expect("a temporary folder");
        let development = Path::new("/repository/assets/kits");
        assert_eq!(
            kits_dir(Some(exe_dir.path()), Some(development)),
            development
        );
    }

    #[test]
    fn without_kits_next_to_the_executable_a_release_build_uses_that_folder_all_the_same() {
        let exe_dir = tempfile::tempdir().expect("a temporary folder");
        assert_eq!(
            kits_dir(Some(exe_dir.path()), None),
            exe_dir.path().join("kits")
        );
    }

    #[test]
    fn the_development_folder_is_the_repository_kits_folder_in_development_builds() {
        let dir = development_kits_dir();
        assert_eq!(dir.is_some(), cfg!(debug_assertions));
        if let Some(dir) = dir {
            assert!(dir.join("acoustic").join("kit.json").is_file(), "{dir:?}");
        }
    }

    #[test]
    fn the_log_is_in_the_logs_folder_of_the_local_data() {
        let paths = AppPaths {
            config_dir: PathBuf::from("config"),
            local_data_dir: PathBuf::from("local"),
            kits_dir: PathBuf::from("kits"),
        };
        assert_eq!(paths.logs_dir(), Path::new("local").join("logs"));
    }
}
