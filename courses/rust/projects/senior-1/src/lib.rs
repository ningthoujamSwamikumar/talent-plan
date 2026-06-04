//! Phase 6, Project 1: API Design and Versioning
//!
//! This crate implements a versioned Task Management API with OpenAPI documentation,
//! Swagger UI, URL path versioning (v1 and v2), backward-compatible evolution,
//! deprecation headers, and SDK generation support.

pub mod openapi;
pub mod v1;
pub mod v2;

use axum::Router;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use utoipa::ToSchema;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Internal (superset) task model — shared between v1 and v2
// ---------------------------------------------------------------------------

/// The internal representation of a task. Contains all fields across all API
/// versions. Version-specific handlers convert to/from their own public types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalTask {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // v2-only fields
    pub tags: Vec<String>,
    pub due_date: Option<DateTime<Utc>>,
}

/// Task status enum used across versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

/// Task priority enum used across versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// ---------------------------------------------------------------------------
// Shared application state
// ---------------------------------------------------------------------------

/// Shared state that both v1 and v2 handlers use.
#[derive(Debug, Clone)]
pub struct AppState {
    pub tasks: Arc<RwLock<HashMap<Uuid, InternalTask>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Router construction
// ---------------------------------------------------------------------------

/// Build the complete application router with v1, v2, OpenAPI spec, and Swagger UI.
///
/// TODO: Students implement this function.
///
/// Requirements:
/// - Nest v1 handlers under `/v1`
/// - Nest v2 handlers under `/v2`
/// - Serve the OpenAPI JSON spec at `/api-doc/openapi.json`
/// - Mount Swagger UI at `/swagger-ui/`
/// - Apply deprecation header middleware to the v1 sub-router
/// - Attach `AppState` as shared state
pub fn app() -> Router {
    todo!("Build the complete application router")
}

/// Start the server on the given address.
pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app()).await?;
    Ok(())
}
