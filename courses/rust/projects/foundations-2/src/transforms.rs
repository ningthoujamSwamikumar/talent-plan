use std::str::FromStr;

use crate::error::{CsvParseError, JsonError, PipelineError};

// ---------------------------------------------------------------------------
// Core trait
// ---------------------------------------------------------------------------

/// A data transform that converts one string into another.
///
/// Each implementation declares its own `Error` associated type. When the
/// transform is used inside a [`Pipeline`](crate::Pipeline), the error type
/// must be [`PipelineError`] (or convertible to it).
pub trait Transform {
    /// The error type this transform can produce.
    type Error;

    /// Apply the transform to `input` and return the result.
    fn transform(&self, input: &str) -> Result<String, Self::Error>;

    /// A human-readable name for this transform (e.g. `"uppercase"`).
    fn name(&self) -> &str;
}

// ---------------------------------------------------------------------------
// Uppercase
// ---------------------------------------------------------------------------

/// Converts the entire input string to uppercase.
///
/// This transform is infallible -- it always succeeds on valid `&str`.
pub struct Uppercase;

impl Transform for Uppercase {
    type Error = PipelineError;

    /// Return the input converted to uppercase.
    fn transform(&self, input: &str) -> Result<String, Self::Error> {
        todo!("Implement Uppercase::transform")
    }

    fn name(&self) -> &str {
        "uppercase"
    }
}

// ---------------------------------------------------------------------------
// Lowercase
// ---------------------------------------------------------------------------

/// Converts the entire input string to lowercase.
pub struct Lowercase;

impl Transform for Lowercase {
    type Error = PipelineError;

    /// Return the input converted to lowercase.
    fn transform(&self, input: &str) -> Result<String, Self::Error> {
        todo!("Implement Lowercase::transform")
    }

    fn name(&self) -> &str {
        "lowercase"
    }
}

// ---------------------------------------------------------------------------
// TrimWhitespace
// ---------------------------------------------------------------------------

/// Trims leading and trailing whitespace from the input.
pub struct TrimWhitespace;

impl Transform for TrimWhitespace {
    type Error = PipelineError;

    /// Return the input with leading/trailing whitespace removed.
    fn transform(&self, input: &str) -> Result<String, Self::Error> {
        todo!("Implement TrimWhitespace::transform")
    }

    fn name(&self) -> &str {
        "trim_whitespace"
    }
}

// ---------------------------------------------------------------------------
// CsvToJson
// ---------------------------------------------------------------------------

/// Parses CSV input (header row + data rows) and produces a JSON array of
/// objects.
///
/// # Format
///
/// * The first line contains comma-separated header names.
/// * Each subsequent non-empty line contains comma-separated values.
/// * A row whose field count differs from the header count is an error.
///
/// # Example
///
/// ```text
/// name,age
/// Alice,30
/// Bob,25
/// ```
///
/// produces `[{"name":"Alice","age":"30"},{"name":"Bob","age":"25"}]`.
pub struct CsvToJson;

impl Transform for CsvToJson {
    type Error = PipelineError;

    /// Parse CSV `input` and return a JSON array string.
    fn transform(&self, input: &str) -> Result<String, Self::Error> {
        todo!("Implement CsvToJson::transform")
    }

    fn name(&self) -> &str {
        "csv_to_json"
    }
}

// ---------------------------------------------------------------------------
// JsonPrettify
// ---------------------------------------------------------------------------

/// Takes compact JSON and re-serializes it with indentation.
pub struct JsonPrettify;

impl Transform for JsonPrettify {
    type Error = PipelineError;

    /// Parse `input` as JSON and return a pretty-printed version.
    fn transform(&self, input: &str) -> Result<String, Self::Error> {
        todo!("Implement JsonPrettify::transform")
    }

    fn name(&self) -> &str {
        "json_prettify"
    }
}

// ---------------------------------------------------------------------------
// TransformKind enum (Part 5)
// ---------------------------------------------------------------------------

/// An enum representing the available transform types.
///
/// Implement [`FromStr`] so that `"uppercase".parse::<TransformKind>()` works.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformKind {
    Uppercase,
    Lowercase,
    TrimWhitespace,
    CsvToJson,
    JsonPrettify,
}

impl FromStr for TransformKind {
    type Err = PipelineError;

    /// Parse a string like `"uppercase"` into the corresponding variant.
    ///
    /// Return `PipelineError::UnknownTransform` for unrecognized input.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Implement FromStr for TransformKind")
    }
}

// ---------------------------------------------------------------------------
// TransformSpec (Part 5)
// ---------------------------------------------------------------------------

/// A specification that describes a transform to apply.
///
/// Implement `TryFrom<&str>` to parse a string into a `TransformSpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformSpec {
    pub kind: TransformKind,
}

impl TryFrom<&str> for TransformSpec {
    type Error = PipelineError;

    /// Parse a transform name (e.g. `"csv_to_json"`) into a `TransformSpec`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!("Implement TryFrom<&str> for TransformSpec")
    }
}
