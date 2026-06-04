pub mod models;
pub mod repository;
pub mod grpc;
pub mod graphql;

use std::sync::Arc;
use tokio::sync::broadcast;

use models::TaskChangeEvent;
use repository::{TaskRepository, ProjectRepository};

/// Shared application state, passed to all API surfaces (REST, gRPC, GraphQL).
#[derive(Clone)]
pub struct AppState {
    pub task_repo: Arc<dyn TaskRepository>,
    pub project_repo: Arc<dyn ProjectRepository>,
    pub event_tx: broadcast::Sender<TaskChangeEvent>,
}

impl AppState {
    pub fn new(
        task_repo: Arc<dyn TaskRepository>,
        project_repo: Arc<dyn ProjectRepository>,
    ) -> Self {
        let (event_tx, _) = broadcast::channel(256);
        Self {
            task_repo,
            project_repo,
            event_tx,
        }
    }
}
