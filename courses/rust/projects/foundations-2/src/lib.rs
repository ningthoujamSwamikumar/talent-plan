//! # Type Machinist
//!
//! A composable data transformation pipeline that demonstrates Rust's trait
//! system, generics, associated types, error handling, and standard conversion
//! traits.
//!
//! ## Architecture
//!
//! ```text
//! input ──> [Transform 1] ──> [Transform 2] ──> ... ──> output
//! ```
//!
//! Each box is a type implementing the [`Transform`] trait. The [`Pipeline`]
//! chains them together, and the [`PipelineBuilder`] provides a fluent API for
//! constructing pipelines.

pub mod error;
pub mod pipeline;
pub mod transforms;

// Re-export the main public types for convenience.
pub use error::PipelineError;
pub use pipeline::{Pipeline, PipelineBuilder};
pub use transforms::{
    CsvToJson, JsonPrettify, Lowercase, Transform, TransformKind, TransformSpec, TrimWhitespace,
    Uppercase,
};
