//! V2 Task API handlers.
//!
//! V2 extends the task model with `tags` (Vec<String>) and `due_date` (Option<DateTime>).

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
// V2 public models
// ---------------------------------------------------------------------------

/// V2 task response — includes tags and due_date.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaskV2 {
    pub id: Uuid,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// V2 request body for creating a task.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateTaskV2 {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub tags: Option<Vec<String>>,
    pub due_date: Option<DateTime<Utc>>,
}

/// V2 request body for updating a task.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateTaskV2 {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
    pub tags: Option<Vec<String>>,
    pub due_date: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Conversions
// ---------------------------------------------------------------------------

impl From<&InternalTask> for TaskV2 {
    fn from(t: &InternalTask) -> Self {
        Self {
            id: t.id,
            title: t.title.clone(),
            description: t.description.clone(),
            status: t.status,
            priority: t.priority,
            tags: t.tags.clone(),
            due_date: t.due_date,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Handlers — students must implement the bodies
// ---------------------------------------------------------------------------

/// List all tasks (v2).
///
/// TODO: Return all tasks converted to `TaskV2`.
#[utoipa::path(
    get,
    path = "/v2/tasks",
    tag = "tasks-v2",
    responses(
        (status = 200, description = "List of tasks", body = Vec<TaskV2>)
    )
)]
pub async fn list_tasks(State(_state): State<AppState>) -> Json<Vec<TaskV2>> {
    todo!("Implement v2 list_tasks")
}

/// Get a single task by ID (v2).
///
/// TODO: Look up the task by UUID. Return 404 if not found.
#[utoipa::path(
    get,
    path = "/v2/tasks/{id}",
    tag = "tasks-v2",
    params(("id" = Uuid, Path, description = "Task ID")),
    responses(
        (status = 200, description = "Task found", body = TaskV2),
        (status = 404, description = "Task not found")
    )
)]
pub async fn get_task(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<TaskV2>, StatusCode> {
    todo!("Implement v2 get_task")
}

/// Create a new task (v2).
///
/// TODO: Create an `InternalTask` populating tags and due_date from the request.
#[utoipa::path(
    post,
    path = "/v2/tasks",
    tag = "tasks-v2",
    request_body = CreateTaskV2,
    responses(
        (status = 201, description = "Task created", body = TaskV2)
    )
)]
pub async fn create_task(
    State(_state): State<AppState>,
    Json(_body): Json<CreateTaskV2>,
) -> (StatusCode, Json<TaskV2>) {
    todo!("Implement v2 create_task")
}

/// Update an existing task (v2).
///
/// TODO: Merge update fields including tags and due_date, update `updated_at`.
#[utoipa::path(
    put,
    path = "/v2/tasks/{id}",
    tag = "tasks-v2",
    params(("id" = Uuid, Path, description = "Task ID")),
    request_body = UpdateTaskV2,
    responses(
        (status = 200, description = "Task updated", body = TaskV2),
        (status = 404, description = "Task not found")
    )
)]
pub async fn update_task(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_body): Json<UpdateTaskV2>,
) -> Result<Json<TaskV2>, StatusCode> {
    todo!("Implement v2 update_task")
}

/// Delete a task (v2).
///
/// TODO: Remove the task from storage. Return 404 if not found.
#[utoipa::path(
    delete,
    path = "/v2/tasks/{id}",
    tag = "tasks-v2",
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
    todo!("Implement v2 delete_task")
}

/// Build the v2 router.
///
/// TODO: Wire up all the v2 handlers to their routes.
pub fn v2_router() -> axum::Router<AppState> {
    todo!("Build the v2 router with all CRUD routes")
}
