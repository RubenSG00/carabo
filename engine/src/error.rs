//! The engine's error type (06 §20). There is one variant for each error code of 04 §6.3 that
//! the engine can produce, each with a technical detail for the log. `ALREADY_EXISTS`,
//! `NAME_INVALID` and `BUSY` are produced only by the native program (04 §6.2).

use std::io;
use std::path::Path;

use thiserror::Error;

/// An error returned by the engine.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EngineError {
    /// A file could not be read or written.
    #[error("IO: {0}")]
    Io(String),
    /// A file that must exist does not.
    #[error("NOT_FOUND: {0}")]
    NotFound(String),
    /// A file does not have the expected format.
    #[error("UNSUPPORTED_FORMAT: {0}")]
    UnsupportedFormat(String),
    /// The caller cancelled the operation.
    #[error("CANCELLED: {0}")]
    Cancelled(String),
    /// The snapshot to export has no segments.
    #[error("NOTHING_TO_EXPORT: {0}")]
    NothingToExport(String),
    /// There is no output stream.
    #[error("NO_OUTPUT_DEVICE: {0}")]
    NoOutputDevice(String),
    /// There is no input stream.
    #[error("NO_INPUT_DEVICE: {0}")]
    NoInputDevice(String),
    /// The round-trip delay could not be measured.
    #[error("CALIBRATION_FAILED: {0}")]
    CalibrationFailed(String),
    /// The snapshot breaks a condition of 06 §5.5.
    #[error("INVALID_SNAPSHOT: {0}")]
    InvalidSnapshot(String),
    /// Anything else, including what is not implemented yet.
    #[error("ENGINE_INTERNAL: {0}")]
    EngineInternal(String),
}

impl EngineError {
    /// The error code, with the exact text of 04 §6.3.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Io(_) => "IO",
            Self::NotFound(_) => "NOT_FOUND",
            Self::UnsupportedFormat(_) => "UNSUPPORTED_FORMAT",
            Self::Cancelled(_) => "CANCELLED",
            Self::NothingToExport(_) => "NOTHING_TO_EXPORT",
            Self::NoOutputDevice(_) => "NO_OUTPUT_DEVICE",
            Self::NoInputDevice(_) => "NO_INPUT_DEVICE",
            Self::CalibrationFailed(_) => "CALIBRATION_FAILED",
            Self::InvalidSnapshot(_) => "INVALID_SNAPSHOT",
            Self::EngineInternal(_) => "ENGINE_INTERNAL",
        }
    }

    /// The technical detail, for the log.
    pub fn detail(&self) -> &str {
        match self {
            Self::Io(detail)
            | Self::NotFound(detail)
            | Self::UnsupportedFormat(detail)
            | Self::Cancelled(detail)
            | Self::NothingToExport(detail)
            | Self::NoOutputDevice(detail)
            | Self::NoInputDevice(detail)
            | Self::CalibrationFailed(detail)
            | Self::InvalidSnapshot(detail)
            | Self::EngineInternal(detail) => detail,
        }
    }

    /// The error of a failed file operation on `path`: `NOT_FOUND` if the file does not exist,
    /// `IO` otherwise, as the native program converts its own (07 §6.2).
    pub fn from_io(path: &Path, error: &io::Error) -> Self {
        let detail = format!("{}: {error}", path.display());
        if error.kind() == io::ErrorKind::NotFound {
            Self::NotFound(detail)
        } else {
            Self::Io(detail)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The codes of 04 §6.3.
    const BRIDGE_ERROR_CODES: [&str; 13] = [
        "IO",
        "NOT_FOUND",
        "ALREADY_EXISTS",
        "NAME_INVALID",
        "UNSUPPORTED_FORMAT",
        "BUSY",
        "CANCELLED",
        "NOTHING_TO_EXPORT",
        "NO_OUTPUT_DEVICE",
        "NO_INPUT_DEVICE",
        "CALIBRATION_FAILED",
        "INVALID_SNAPSHOT",
        "ENGINE_INTERNAL",
    ];

    fn every_variant(detail: &str) -> [EngineError; 10] {
        let d = || detail.to_string();
        [
            EngineError::Io(d()),
            EngineError::NotFound(d()),
            EngineError::UnsupportedFormat(d()),
            EngineError::Cancelled(d()),
            EngineError::NothingToExport(d()),
            EngineError::NoOutputDevice(d()),
            EngineError::NoInputDevice(d()),
            EngineError::CalibrationFailed(d()),
            EngineError::InvalidSnapshot(d()),
            EngineError::EngineInternal(d()),
        ]
    }

    #[test]
    fn every_code_is_a_bridge_error_code_and_no_two_variants_share_one() {
        let codes: Vec<&str> = every_variant("").iter().map(EngineError::code).collect();
        for code in &codes {
            assert!(BRIDGE_ERROR_CODES.contains(code), "{code}");
        }
        let mut unique = codes.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), codes.len());
    }

    #[test]
    fn the_engine_produces_every_code_but_those_of_the_native_program() {
        let codes: Vec<&str> = every_variant("").iter().map(EngineError::code).collect();
        let native_only = ["ALREADY_EXISTS", "NAME_INVALID", "BUSY"];
        for code in BRIDGE_ERROR_CODES {
            assert_eq!(
                codes.contains(&code),
                !native_only.contains(&code),
                "{code}"
            );
        }
    }

    #[test]
    fn keeps_the_detail_and_shows_it_after_the_code() {
        for error in every_variant("kit.json is missing") {
            assert_eq!(error.detail(), "kit.json is missing");
            assert_eq!(
                error.to_string(),
                format!("{}: kit.json is missing", error.code())
            );
        }
    }

    #[test]
    fn a_missing_file_is_not_found_and_any_other_io_error_is_io() {
        let path = Path::new("kits/acoustic/kit.json");
        let missing = io::Error::from(io::ErrorKind::NotFound);
        let denied = io::Error::from(io::ErrorKind::PermissionDenied);
        assert_eq!(EngineError::from_io(path, &missing).code(), "NOT_FOUND");
        assert_eq!(EngineError::from_io(path, &denied).code(), "IO");
        assert!(EngineError::from_io(path, &denied)
            .detail()
            .starts_with("kits/acoustic/kit.json: "));
    }
}
