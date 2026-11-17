//! The error type of the bridge (04 §6.3). Every command returns `Result<T, BridgeError>`, and
//! the interface receives the error as `{ code, detail }` (07 §6.2).

use std::io;

use carabo_engine::EngineError;
use serde::Serialize;

/// The error codes of 04 §6.3, serialized with their exact texts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BridgeErrorCode {
    Io,
    NotFound,
    AlreadyExists,
    NameInvalid,
    UnsupportedFormat,
    Busy,
    Cancelled,
    NothingToExport,
    NoOutputDevice,
    NoInputDevice,
    CalibrationFailed,
    InvalidSnapshot,
    EngineInternal,
}

/// The error with which a bridge command fails (04 §6.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BridgeError {
    pub code: BridgeErrorCode,
    /// Technical text for the log; it is never shown to the user.
    pub detail: String,
}

impl BridgeError {
    /// An error with this code and detail.
    pub fn new(code: BridgeErrorCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }
}

/// 07 §6.2: an I/O error whose kind is "not found" is `NOT_FOUND`; any other, `IO`.
impl From<io::Error> for BridgeError {
    fn from(error: io::Error) -> Self {
        let code = if error.kind() == io::ErrorKind::NotFound {
            BridgeErrorCode::NotFound
        } else {
            BridgeErrorCode::Io
        };
        Self::new(code, error.to_string())
    }
}

/// Each engine error keeps its code of 04 §6.3 and its detail.
impl From<EngineError> for BridgeError {
    fn from(error: EngineError) -> Self {
        let code = match &error {
            EngineError::Io(_) => BridgeErrorCode::Io,
            EngineError::NotFound(_) => BridgeErrorCode::NotFound,
            EngineError::UnsupportedFormat(_) => BridgeErrorCode::UnsupportedFormat,
            EngineError::Cancelled(_) => BridgeErrorCode::Cancelled,
            EngineError::NothingToExport(_) => BridgeErrorCode::NothingToExport,
            EngineError::NoOutputDevice(_) => BridgeErrorCode::NoOutputDevice,
            EngineError::NoInputDevice(_) => BridgeErrorCode::NoInputDevice,
            EngineError::CalibrationFailed(_) => BridgeErrorCode::CalibrationFailed,
            EngineError::InvalidSnapshot(_) => BridgeErrorCode::InvalidSnapshot,
            EngineError::EngineInternal(_) => BridgeErrorCode::EngineInternal,
        };
        Self::new(code, error.detail())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The text of a code, as the interface receives it.
    fn code_text(code: BridgeErrorCode) -> String {
        serde_json::to_value(code)
            .ok()
            .and_then(|value| value.as_str().map(str::to_string))
            .unwrap_or_default()
    }

    #[test]
    fn serializes_with_the_fields_code_and_detail_only() {
        let error = BridgeError::new(BridgeErrorCode::NoOutputDevice, "no output stream");
        assert_eq!(
            serde_json::to_value(&error).expect("a bridge error serializes"),
            json!({ "code": "NO_OUTPUT_DEVICE", "detail": "no output stream" })
        );
    }

    #[test]
    fn every_code_serializes_with_its_text_of_04_6_3() {
        let codes = [
            (BridgeErrorCode::Io, "IO"),
            (BridgeErrorCode::NotFound, "NOT_FOUND"),
            (BridgeErrorCode::AlreadyExists, "ALREADY_EXISTS"),
            (BridgeErrorCode::NameInvalid, "NAME_INVALID"),
            (BridgeErrorCode::UnsupportedFormat, "UNSUPPORTED_FORMAT"),
            (BridgeErrorCode::Busy, "BUSY"),
            (BridgeErrorCode::Cancelled, "CANCELLED"),
            (BridgeErrorCode::NothingToExport, "NOTHING_TO_EXPORT"),
            (BridgeErrorCode::NoOutputDevice, "NO_OUTPUT_DEVICE"),
            (BridgeErrorCode::NoInputDevice, "NO_INPUT_DEVICE"),
            (BridgeErrorCode::CalibrationFailed, "CALIBRATION_FAILED"),
            (BridgeErrorCode::InvalidSnapshot, "INVALID_SNAPSHOT"),
            (BridgeErrorCode::EngineInternal, "ENGINE_INTERNAL"),
        ];
        for (code, text) in codes {
            assert_eq!(code_text(code), text);
        }
    }

    #[test]
    fn an_io_error_of_kind_not_found_is_not_found() {
        let error = BridgeError::from(io::Error::new(io::ErrorKind::NotFound, "no such file"));
        assert_eq!(
            error,
            BridgeError::new(BridgeErrorCode::NotFound, "no such file")
        );
    }

    #[test]
    fn any_other_io_error_is_io() {
        for kind in [
            io::ErrorKind::PermissionDenied,
            io::ErrorKind::AlreadyExists,
            io::ErrorKind::InvalidData,
            io::ErrorKind::Other,
        ] {
            let error = BridgeError::from(io::Error::new(kind, "cannot write"));
            assert_eq!(error, BridgeError::new(BridgeErrorCode::Io, "cannot write"));
        }
    }

    #[test]
    fn each_engine_error_keeps_its_code_and_its_detail() {
        let detail = || "technical detail".to_string();
        let errors = [
            EngineError::Io(detail()),
            EngineError::NotFound(detail()),
            EngineError::UnsupportedFormat(detail()),
            EngineError::Cancelled(detail()),
            EngineError::NothingToExport(detail()),
            EngineError::NoOutputDevice(detail()),
            EngineError::NoInputDevice(detail()),
            EngineError::CalibrationFailed(detail()),
            EngineError::InvalidSnapshot(detail()),
            EngineError::EngineInternal(detail()),
        ];
        for engine_error in errors {
            let code = engine_error.code();
            let error = BridgeError::from(engine_error);
            assert_eq!(code_text(error.code), code);
            assert_eq!(error.detail, "technical detail");
        }
    }
}
