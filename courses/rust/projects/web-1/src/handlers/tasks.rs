//! Handlers for task CRUD operations.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::error::AppError;
use crate::models::{CreateTask, TaskFilterParams, UpdateTask};
use crate::store::AppState;

/// `POST /projects/:project_id/tasks` -- create a task under a project.
///
/// Returns `201 Created` with the task JSON on success.
pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
    Json(body): Json<CreateTask>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let task = state.create_task(project_id, body)?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// `GET /projects/:project_id/tasks` -- list tasks for a project.
///
/// Supports pagination (`page`, `per_page`) and filtering (`status`, `priority`)
/// via query parameters.
pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
    Query(filters): Query<TaskFilterParams>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.list_tasks(project_id, &filters)?;
    Ok(Json(response))
}

/// `GET /tasks/:id` -- get a single task.
pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = state.get_task(id)?;
    Ok(Json(task))
}

/// `PUT /tasks/:id` -- update a task.
pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTask>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let task = state.update_task(id, body)?;
    Ok(Json(task))
}

/// `DELETE /tasks/:id` -- delete a task.
///
/// Returns `204 No Content` on success.
pub async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_task(id)?;
    Ok(StatusCode::NO_CONTENT)
}
