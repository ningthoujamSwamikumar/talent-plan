//! Domain models, request/response types, and pagination helpers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ---------------------------------------------------------------------------
// Domain entities
// ---------------------------------------------------------------------------

/// A project is a top-level container for related tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique identifier.
    pub id: Uuid,
    /// Human-readable name.
    pub name: String,
    /// Optional longer description.
    pub description: Option<String>,
    /// When the project was created.
    pub created_at: DateTime<Utc>,
    /// When the project was last modified.
    pub updated_at: DateTime<Utc>,
}

/// A task belongs to a project and tracks a unit of work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier.
    pub id: Uuid,
    /// The project this task belongs to.
    pub project_id: Uuid,
    /// Short summary of the task.
    pub title: String,
    /// Optional longer description.
    pub description: Option<String>,
    /// Current status.
    pub status: TaskStatus,
    /// Priority level.
    pub priority: Priority,
    /// When the task was created.
    pub created_at: DateTime<Utc>,
    /// When the task was last modified.
    pub updated_at: DateTime<Utc>,
}

/// The lifecycle status of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Not yet started.
    Todo,
    /// Currently being worked on.
    InProgress,
    /// Completed.
    Done,
}

/// Priority level for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    /// Low priority.
    Low,
    /// Medium priority.
    Medium,
    /// High priority.
    High,
    /// Critical -- must be addressed immediately.
    Critical,
}

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Body for `POST /projects`.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProject {
    /// Project name (must not be empty).
    #[validate(length(min = 1, message = "name must not be empty"))]
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
}

/// Body for `PUT /projects/:id`.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProject {
    /// New name (if provided, must not be empty).
    #[validate(length(min = 1, message = "name must not be empty"))]
    pub name: Option<String>,
    /// New description.
    pub description: Option<String>,
}

/// Body for `POST /projects/:project_id/tasks`.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTask {
    /// Task title (must not be empty).
    #[validate(length(min = 1, message = "title must not be empty"))]
    pub title: String,
    /// Optional description.
    pub description: Option<String>,
    /// Initial status (defaults to `Todo` if omitted).
    pub status: Option<TaskStatus>,
    /// Priority (defaults to `Medium` if omitted).
    pub priority: Option<Priority>,
}

/// Body for `PUT /tasks/:id`.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTask {
    /// New title (if provided, must not be empty).
    #[validate(length(min = 1, message = "title must not be empty"))]
    pub title: Option<String>,
    /// New description.
    pub description: Option<String>,
    /// New status.
    pub status: Option<TaskStatus>,
    /// New priority.
    pub priority: Option<Priority>,
}

// ---------------------------------------------------------------------------
// Query / pagination
// ---------------------------------------------------------------------------

/// Query parameters for list endpoints that support pagination.
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed). Defaults to 1.
    pub page: Option<u32>,
    /// Items per page. Defaults to 10.
    pub per_page: Option<u32>,
}

/// Query parameters for filtering tasks.
#[derive(Debug, Deserialize)]
pub struct TaskFilterParams {
    /// Filter by page number.
    pub page: Option<u32>,
    /// Items per page.
    pub per_page: Option<u32>,
    /// Filter by status.
    pub status: Option<TaskStatus>,
    /// Filter by priority.
    pub priority: Option<Priority>,
}

/// A paginated response wrapper.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items on this page.
    pub data: Vec<T>,
    /// Total number of items across all pages.
    pub total: usize,
    /// Current page number (1-indexed).
    pub page: u32,
    /// Items per page.
    pub per_page: u32,
}
