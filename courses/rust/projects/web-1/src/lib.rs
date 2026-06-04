#![deny(missing_docs)]
//! TaskForge API -- a task management REST API built with axum.

pub mod error;
pub mod handlers;
pub mod models;
pub mod store;

use axum::Router;
use std::sync::Arc;
use store::AppState;

/// Creates the application router with all routes and middleware.
///
/// Students should:
/// 1. Define routes for health, projects, and tasks.
/// 2. Attach the shared `AppState` via `.with_state()`.
/// 3. Add middleware layers (CORS, tracing, request ID).
pub fn app(_state: Arc<AppState>) -> Router {
    todo!("build the Router with routes and middleware")
}
