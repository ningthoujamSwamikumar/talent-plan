//! V1 Task API handlers.
//!
//! These handlers expose the original task model without tags or due_date.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{AppState, InternalTask, Priority, TaskStatus};

// ---------------------------------------------------------------------------
// V1 public models
// ---------------------------------------------------------------------------

/// V1 task response — does NOT include tags or due_date.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaskV1 {
    pub id: Uuid,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// V1 request body for creating a task.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateTaskV1 {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
}

/// V1 request body for updating a task.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateTaskV1 {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
}

// ---------------------------------------------------------------------------
// Conversions
// ---------------------------------------------------------------------------

impl From<&InternalTask> for TaskV1 {
    fn from(t: &InternalTask) -> Self {
        Self {
            id: t.id,
            title: t.title.clone(),
            description: t.description.clone(),
            status: t.status,
            priority: t.priority,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Handlers — students must implement the bodies
// ---------------------------------------------------------------------------

/// List all tasks (v1).
///
/// TODO: Return all tasks converted to `TaskV1`.
#[utoipa::path(
    get,
    path = "/v1/tasks",
    tag = "tasks-v1",
    responses(
        (status = 200, description = "List of tasks", body = Vec<TaskV1>)
    )
)]
pub async fn list_tasks(State(_state): State<AppState>) -> Json<Vec<TaskV1>> {
    todo!("Implement v1 list_tasks")
}

/// Get a single task by ID (v1).
///
/// TODO: Look up the task by UUID. Return 404 if not found.
#[utoipa::path(
    get,
    path = "/v1/tasks/{id}",
    tag = "tasks-v1",
    params(("id" = Uuid, Path, description = "Task ID")),
    responses(
        (status = 200, description = "Task found", body = TaskV1),
        (status = 404, description = "Task not found")
    )
)]
pub async fn get_task(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<TaskV1>, StatusCode> {
    todo!("Implement v1 get_task")
}

/// Create a new task (v1).
///
/// TODO: Create an `InternalTask` with default values for v2 fields (empty tags, None due_date).
#[utoipa::path(
    post,
    path = "/v1/tasks",
    tag = "tasks-v1",
    request_body = CreateTaskV1,
    responses(
        (status = 201, description = "Task created", body = TaskV1)
    )
)]
pub async fn create_task(
    State(_state): State<AppState>,
    Json(_body): Json<CreateTaskV1>,
) -> (StatusCode, Json<TaskV1>) {
    todo!("Implement v1 create_task")
}

/// Update an existing task (v1).
///
/// TODO: Merge the update fields, update `updated_at`, return the updated task.
#[utoipa::path(
    put,
    path = "/v1/tasks/{id}",
    tag = "tasks-v1",
    params(("id" = Uuid, Path, description = "Task ID")),
    request_body = UpdateTaskV1,
    responses(
        (status = 200, description = "Task updated", body = TaskV1),
        (status = 404, description = "Task not found")
    )
)]
pub async fn update_task(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_body): Json<UpdateTaskV1>,
) -> Result<Json<TaskV1>, StatusCode> {
    todo!("Implement v1 update_task")
}

/// Delete a task (v1).
///
/// TODO: Remove the task from storage. Return 404 if not found.
#[utoipa::path(
    delete,
    path = "/v1/tasks/{id}",
    tag = "tasks-v1",
    params(("id" = Uuid, Path, description = "Task ID")),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Task not found")
    )
)]
pub async fn delete_task(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> StatusCode {
    todo!("Implement v1 delete_task")
}

/// Build the v1 router.
///
/// TODO: Wire up all the v1 handlers to their routes.
pub fn v1_router() -> axum::Router<AppState> {
    todo!("Build the v1 router with all CRUD routes")
}
