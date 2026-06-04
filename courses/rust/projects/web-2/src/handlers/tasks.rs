use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{CreateTask, MoveTasksRequest, TaskFilter, UpdateTask};
use crate::repository::TaskRepository;

/// Mount all task routes under `/tasks`.
pub fn task_routes() -> Router<TaskRepository> {
    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/search", get(search_tasks))
        .route("/tasks/move", post(move_tasks))
        .route(
            "/tasks/:id",
            get(get_task).put(update_task).delete(delete_task),
        )
        .route("/projects/:project_id/tasks", get(list_project_tasks))
        .route(
            "/projects/:project_id/tasks/stats",
            get(task_stats_by_project),
        )
}

/// POST /tasks
async fn create_task(
    State(repo): State<TaskRepository>,
    Json(body): Json<CreateTask>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    todo!()
}

/// GET /tasks/:id
async fn get_task(
    State(repo): State<TaskRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// GET /projects/:project_id/tasks
async fn list_project_tasks(
    State(repo): State<TaskRepository>,
    Path(project_id): Path<Uuid>,
    Query(filter): Query<TaskFilter>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// GET /tasks/search?search=keyword
async fn search_tasks(
    State(repo): State<TaskRepository>,
    Query(filter): Query<TaskFilter>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// PUT /tasks/:id
async fn update_task(
    State(repo): State<TaskRepository>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTask>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// DELETE /tasks/:id
async fn delete_task(
    State(repo): State<TaskRepository>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    todo!()
}

/// POST /tasks/move
async fn move_tasks(
    State(repo): State<TaskRepository>,
    Json(body): Json<MoveTasksRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}

/// GET /projects/:project_id/tasks/stats
async fn task_stats_by_project(
    State(repo): State<TaskRepository>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!()
}
