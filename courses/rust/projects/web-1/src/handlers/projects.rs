//! Handlers for project CRUD operations.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::error::AppError;
use crate::models::{CreateProject, UpdateProject};
use crate::store::AppState;

/// `POST /projects` -- create a new project.
///
/// Returns `201 Created` with the project JSON on success, or `422` if
/// validation fails.
pub async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateProject>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let project = state.create_project(body)?;
    Ok((StatusCode::CREATED, Json(project)))
}

/// `GET /projects` -- list all projects.
pub async fn list_projects(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let projects = state.list_projects()?;
    Ok(Json(projects))
}

/// `GET /projects/:id` -- get a single project.
pub async fn get_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let project = state.get_project(id)?;
    Ok(Json(project))
}

/// `PUT /projects/:id` -- update a project.
pub async fn update_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProject>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let project = state.update_project(id, body)?;
    Ok(Json(project))
}

/// `DELETE /projects/:id` -- delete a project and its tasks.
///
/// Returns `204 No Content` on success.
pub async fn delete_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_project(id)?;
    Ok(StatusCode::NO_CONTENT)
}
