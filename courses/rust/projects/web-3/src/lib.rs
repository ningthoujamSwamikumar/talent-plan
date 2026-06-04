pub mod auth;
pub mod error;
pub mod handlers;
pub mod models;
pub mod repository;

// Re-export key types for convenience.
pub use error::AppError;
pub use models::Role;
