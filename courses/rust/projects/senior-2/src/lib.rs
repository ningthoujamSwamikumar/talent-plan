//! Phase 6, Project 2: Security Hardening
//!
//! This crate implements security hardening for a note-taking API: input sanitization,
//! SQL injection prevention, security headers, HMAC request signing, audit logging,
//! and secrets management.

pub mod audit;
pub mod hmac_auth;
pub mod sanitize;
pub mod secrets;
pub mod security_headers;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Note model
// ---------------------------------------------------------------------------

/// A note in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request body for creating a note.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
    pub author: String,
}

/// Request body for updating a note.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
}

// ---------------------------------------------------------------------------
// NoteStore trait — Part 2: SQL Injection Prevention
// ---------------------------------------------------------------------------

/// Trait for looking up notes by title.
///
/// Two implementations demonstrate safe vs. unsafe query patterns.
pub trait NoteStore {
    /// Find notes whose title matches the query.
    fn find_by_title(&self, title: &str) -> Vec<Note>;
}

/// Safe implementation: uses exact-match lookup (simulates parameterized queries).
///
/// TODO: Implement `NoteStore` for `SafeNoteStore`.
/// The lookup should match titles exactly — no string interpolation.
pub struct SafeNoteStore {
    pub notes: HashMap<Uuid, Note>,
}

/// Unsafe implementation: concatenates user input into a "query" string (simulates injection).
///
/// TODO: Implement `NoteStore` for `UnsafeNoteStore`.
/// Demonstrate how concatenating user input can be exploited.
/// For example, if the title contains `" OR 1=1 --`, the simulated query
/// should match all notes (simulating an injection).
pub struct UnsafeNoteStore {
    pub notes: HashMap<Uuid, Note>,
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

/// Shared application state.
#[derive(Debug, Clone)]
pub struct AppState {
    pub notes: Arc<RwLock<HashMap<Uuid, Note>>>,
    pub audit_log: Arc<RwLock<Vec<audit::AuditEvent>>>,
    pub hmac_key: String,
}

impl AppState {
    pub fn new(hmac_key: String) -> Self {
        Self {
            notes: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
            hmac_key,
        }
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// List all notes.
///
/// TODO: Return all notes from state.
pub async fn list_notes(State(_state): State<AppState>) -> Json<Vec<Note>> {
    todo!("Implement list_notes")
}

/// Get a note by ID.
///
/// TODO: Return 404 if not found.
pub async fn get_note(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<Note>, StatusCode> {
    todo!("Implement get_note")
}

/// Create a new note.
///
/// TODO: Sanitize title and content before storing. Log an audit event.
pub async fn create_note(
    State(_state): State<AppState>,
    Json(_body): Json<CreateNote>,
) -> (StatusCode, Json<Note>) {
    todo!("Implement create_note with sanitization and audit logging")
}

/// Update a note.
///
/// TODO: Sanitize updated fields. Log an audit event.
pub async fn update_note(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_body): Json<UpdateNote>,
) -> Result<Json<Note>, StatusCode> {
    todo!("Implement update_note with sanitization and audit logging")
}

/// Delete a note.
///
/// TODO: Remove the note. Log an audit event.
pub async fn delete_note(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> StatusCode {
    todo!("Implement delete_note with audit logging")
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

/// Build the application router with all security layers applied.
///
/// TODO: Students implement this function.
///
/// Requirements:
/// - Wire up CRUD handlers for `/notes` and `/notes/:id`
/// - Apply `security_headers_layer` to all routes
/// - Apply `hmac_auth` middleware to mutation routes (POST, PUT, DELETE)
/// - Apply audit logging middleware
/// - Attach `AppState`
pub fn app(hmac_key: &str) -> Router {
    todo!("Build the secured application router")
}

/// Start the server on the given address.
pub async fn run_server(addr: &str, hmac_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app(hmac_key)).await?;
    Ok(())
}
