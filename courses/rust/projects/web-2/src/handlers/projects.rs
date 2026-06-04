use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{CreateProject, PaginationParams, UpdateProject};
use crate::repository::ProjectRepository;

/// Mount all project routes under `/projects`.
pub fn project_routes() -> Router<ProjectRepository> {
    Router::new()
        .route("/projects", post(create_project).get(list_projects))
        .route(
            "/projects/:id",
            get(get_project).put(update_project).delete(delete_project),
        )
}

/// POST /projects
async fn create_project(
    State(repo): State<ProjectRepository>,
    Json(body): Json<CreateProject>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    todo!()
}

/// GET /projects/:id
async fn get_project(
    State(repo): State<ProjectRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// GET /projects
async fn list_projects(
    State(repo): State<ProjectRepository>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// PUT /projects/:id
async fn update_project(
    State(repo): State<ProjectRepository>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProject>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// DELETE /projects/:id
async fn delete_project(
    State(repo): State<ProjectRepository>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    todo!()
}
