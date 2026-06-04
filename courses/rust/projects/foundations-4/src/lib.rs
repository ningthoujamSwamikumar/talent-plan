#![deny(missing_docs)]
//! # Smart Pointer Workshop
//!
//! Smart pointers, interior mutability, and macros.
//!
//! This crate is the fourth and final foundations project.  It covers:
//!
//! - **`Counted<T>`** -- a custom smart pointer that tracks accesses via
//!   `Deref` / `DerefMut`.
//! - **`Graph<T>`** -- a directed graph built with `Rc<RefCell<...>>`
//!   (single-threaded interior mutability).
//! - **`ConcurrentGraph<T>`** -- a thread-safe graph built with
//!   `Arc<RwLock<...>>`.
//! - **`graph!`** -- a declarative macro for convenient graph construction.
//! - **`#[derive(Describe)]`** -- a procedural derive macro that
//!   auto-generates a `describe()` method.

mod concurrent_graph;
mod counted;
mod graph;

/// A unique node identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

pub use concurrent_graph::ConcurrentGraph;
pub use counted::Counted;
pub use derive_describe::Describe;
pub use graph::Graph;

/// Trait for types that can describe their own structure.
///
/// The [`Describe`](derive_describe::Describe) derive macro generates an
/// implementation that returns a string listing the struct's name, field
/// names, and field types.
pub trait Describe {
    /// Returns a human-readable description of this value's structure.
    ///
    /// For a struct `Point { x: f64, y: f64 }` the output would be:
    /// `"Point { x: f64, y: f64 }"`.
    fn describe(&self) -> String;
}

/// Macro for convenient graph construction.
///
/// Builds a [`Graph<&str>`] from a small DSL of directed edges.
///
/// # Syntax
///
/// ```ignore
/// let g = graph! {
///     "a" => "b",
///     "b" => "c",
///     "c" => "a",
/// };
/// ```
///
/// Each `from => to` pair adds nodes (if not already present) and a
/// directed edge between them.  The resulting graph is returned.
#[macro_export]
macro_rules! graph {
    ($($from:expr => $to:expr),* $(,)?) => {
        todo!("Students implement the graph! macro")
    };
}
