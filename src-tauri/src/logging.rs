//! The log of the native program (05 §7.3, RNF-20): `logs/carabo.log` in the local data folder,
//! one line per entry, `<ISO 8601 UTC date and time> <LEVEL> <module> <message>`.
//!
//! The engine never writes to the log: the native program writes what the engine reports
//! (06 §18.2). No other crate is used to log (07 §2.5).

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

/// Name of the log file in the `logs` folder (05 §7.3).
pub const LOG_FILE_NAME: &str = "carabo.log";

/// Level of a log entry (05 §7.3). The interface sends it in lowercase (`logWrite`, 04 §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Error,
    Warn,
    Info,
}

impl Level {
    /// The text of the level in the log.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
        }
    }
}

/// A log file. Any thread may write to it, except the audio threads, which never log (RNF-20).
#[derive(Debug)]
pub struct Log {
    path: Option<PathBuf>,
    file: Mutex<Option<File>>,
}

impl Log {
    /// Opens `carabo.log` in `logs_dir` to append to it, creating the folder if it does not
    /// exist.
    // TODO(T-051): rotate the file when it reaches 1 MB, keeping 3 files (05 §7.3).
    pub fn open(logs_dir: &Path) -> io::Result<Log> {
        fs::create_dir_all(logs_dir)?;
        let path = logs_dir.join(LOG_FILE_NAME);
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(Log {
            path: Some(path),
            file: Mutex::new(Some(file)),
        })
    }

    /// A log that writes nothing, for when the log file cannot be opened.
    pub fn disabled() -> Log {
        Log {
            path: None,
            file: Mutex::new(None),
        }
    }

    /// Path of the log file, or `None` if the log is disabled.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Appends an entry with the current date and time. A failed write is lost: there is no
    /// other place in which to report it.
    pub fn write(&self, level: Level, module: &str, message: &str) {
        let line = format_line(SystemTime::now(), level, module, message);
        let mut file = self.file.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(file) = file.as_mut() {
            let _ = file.write_all(line.as_bytes());
        }
    }
}

/// The log of the program, once [`install`] has been called.
static LOG: OnceLock<Log> = OnceLock::new();

/// Makes `log` the log of the program. Only the first call has an effect.
pub fn install(log: Log) {
    let _ = LOG.set(log);
}

/// The log of the program; a disabled one if none has been installed.
pub fn log() -> &'static Log {
    LOG.get_or_init(Log::disabled)
}

/// One line of the log, ending in a line break. Line breaks inside `message` become spaces, so
/// that each entry is one line (05 §7.3).
pub fn format_line(time: SystemTime, level: Level, module: &str, message: &str) -> String {
    let message = message.replace(['\r', '\n'], " ");
    format!(
        "{} {} {module} {message}\n",
        format_utc(time),
        level.as_str()
    )
}

/// `time` in ISO 8601, UTC, to the second: `2026-11-17T09:05:03Z`.
fn format_utc(time: SystemTime) -> String {
    // Times before 1970 do not occur; they are written as 1970-01-01T00:00:00Z.
    let seconds = time
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or_default();
    let days = seconds / 86_400;
    let second_of_day = seconds % 86_400;
    let (year, month, day) = civil_date(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        second_of_day / 3_600,
        second_of_day % 3_600 / 60,
        second_of_day % 60
    )
}

/// Year, month and day of the Gregorian calendar of `days` days after 1970-01-01 (the
/// "civil from days" algorithm of Howard Hinnant).
fn civil_date(days: u64) -> (u64, u64, u64) {
    let shifted = days + 719_468;
    let era = shifted / 146_097;
    let day_of_era = shifted % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn at(seconds: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(seconds)
    }

    #[test]
    fn a_line_has_date_and_time_level_module_and_message() {
        // 2026-11-17 09:05:03 UTC.
        let line = format_line(at(1_794_906_303), Level::Warn, "telemetry", "late");
        assert_eq!(line, "2026-11-17T09:05:03Z WARN telemetry late\n");
    }

    #[test]
    fn levels_are_written_in_uppercase() {
        let levels = [Level::Error, Level::Warn, Level::Info].map(Level::as_str);
        assert_eq!(levels, ["ERROR", "WARN", "INFO"]);
    }

    #[test]
    fn levels_are_read_in_lowercase_as_the_interface_sends_them() {
        let levels: Vec<Level> = serde_json::from_str(r#"["error", "warn", "info"]"#)
            .expect("the three levels deserialize");
        assert_eq!(levels, [Level::Error, Level::Warn, Level::Info]);
        assert!(serde_json::from_str::<Level>(r#""ERROR""#).is_err());
    }

    #[test]
    fn dates_are_utc_in_iso_8601() {
        let cases = [
            (0, "1970-01-01T00:00:00Z"),
            (951_782_400, "2000-02-29T00:00:00Z"),
            (951_955_199, "2000-03-01T23:59:59Z"),
            (1_704_067_199, "2023-12-31T23:59:59Z"),
            (1_709_208_000, "2024-02-29T12:00:00Z"),
            (4_102_444_800, "2100-01-01T00:00:00Z"),
        ];
        for (seconds, text) in cases {
            assert_eq!(format_utc(at(seconds)), text, "{seconds}");
        }
    }

    #[test]
    fn each_entry_is_one_line_even_if_the_message_has_line_breaks() {
        let line = format_line(at(0), Level::Error, "ui", "first\r\nsecond\nthird");
        assert_eq!(line, "1970-01-01T00:00:00Z ERROR ui first  second third\n");
    }

    #[test]
    fn writes_the_entries_to_logs_carabo_log_one_line_each_and_appends_on_reopening() {
        let data = tempfile::tempdir().expect("a temporary folder");
        let logs_dir = data.path().join("logs");

        let log = Log::open(&logs_dir).expect("the log opens");
        assert_eq!(log.path(), Some(logs_dir.join("carabo.log").as_path()));
        log.write(Level::Info, "app", "started");
        log.write(Level::Error, "ui", "failed");
        drop(log);
        Log::open(&logs_dir)
            .expect("the log opens again")
            .write(Level::Warn, "telemetry", "again");

        let text = fs::read_to_string(logs_dir.join("carabo.log")).expect("the log exists");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 3);
        for (line, ending) in lines.iter().zip([
            "Z INFO app started",
            "Z ERROR ui failed",
            "Z WARN telemetry again",
        ]) {
            let (date_and_time, rest) = line.split_at(19);
            assert!(is_iso_8601_to_the_second(date_and_time), "{date_and_time}");
            assert_eq!(rest, ending);
        }
    }

    #[test]
    fn a_disabled_log_writes_nothing_and_has_no_path() {
        let log = Log::disabled();
        log.write(Level::Info, "app", "nowhere");
        assert_eq!(log.path(), None);
    }

    /// `YYYY-MM-DDTHH:MM:SS`.
    fn is_iso_8601_to_the_second(text: &str) -> bool {
        text.len() == 19
            && text.char_indices().all(|(index, c)| match index {
                4 | 7 => c == '-',
                10 => c == 'T',
                13 | 16 => c == ':',
                _ => c.is_ascii_digit(),
            })
    }
}
