//! **taskforge-db** -- Phase 2, Project 2: Database Layer with SQLx
//!
//! This crate replaces the in-memory `HashMap` store from web-1 with
//! PostgreSQL, accessed through the [`sqlx`] async driver.  Students learn:
//!
//! * Schema design and migrations
//! * The repository pattern for separating data-access from business logic
//! * Compile-time (and runtime) checked SQL queries
//! * Transactions for atomic multi-step operations
//! * Full-text search, JOINs, and aggregation queries
//! * Connection-pool configuration and health checks

pub mod error;
pub mod handlers;
pub mod models;
pub mod repository;

// Re-exports for convenience
pub use error::AppError;
pub use models::*;
pub use repository::{ProjectRepository, TaskRepository};
