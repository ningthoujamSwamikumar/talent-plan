use crate::models::*;
use std::collections::HashMap;
use uuid::Uuid;

use super::RepositoryError;

/// Trait for task persistence operations.
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, params: CreateTaskParams) -> Result<Task, RepositoryError>;
    async fn get(&self, id: Uuid) -> Result<Task, RepositoryError>;
    async fn list(
        &self,
        filter: TaskFilter,
        pagination: PaginationParams,
    ) -> Result<PaginatedResponse<Task>, RepositoryError>;
    async fn update(&self, id: Uuid, params: UpdateTaskParams) -> Result<Task, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError>;
}

/// Trait for project persistence operations.
#[async_trait::async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn get(&self, id: Uuid) -> Result<Project, RepositoryError>;
    async fn get_many(&self, ids: Vec<Uuid>) -> Result<HashMap<Uuid, Project>, RepositoryError>;
}
