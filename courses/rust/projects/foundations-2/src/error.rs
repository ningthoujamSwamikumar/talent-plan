use thiserror::Error;
use std::fmt;

/// Unified error type for the transformation pipeline.
///
/// Every transform in the pipeline must ultimately produce errors that can be
/// converted into `PipelineError`. The `#[from]` attribute on each variant
/// auto-generates the corresponding `From` implementation, which lets the `?`
/// operator convert seamlessly.
#[derive(Debug, Error)]
pub enum PipelineError {
    /// An error that occurred while parsing CSV input.
    #[error("CSV parse error: {0}")]
    CsvParse(#[from] CsvParseError),

    /// An error that occurred during JSON processing.
    #[error("JSON error: {0}")]
    Json(#[from] JsonError),

    /// An unknown or unsupported transform was requested.
    #[error("unknown transform: {0}")]
    UnknownTransform(String),

    /// A generic / catch-all error with a message.
    #[error("{0}")]
    Other(String),
}

/// Error type for CSV parsing failures.
///
/// Students should add appropriate fields and implement `Display`.
#[derive(Debug)]
pub struct CsvParseError {
    pub message: String,
}

impl fmt::Display for CsvParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CsvParseError {}

/// Error type for JSON processing failures.
///
/// Wraps `serde_json::Error` so that JSON transform errors can flow into
/// `PipelineError` via the `From` chain.
#[derive(Debug)]
pub struct JsonError {
    pub message: String,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JsonError {}

impl From<serde_json::Error> for JsonError {
    fn from(e: serde_json::Error) -> Self {
        JsonError {
            message: e.to_string(),
        }
    }
}
