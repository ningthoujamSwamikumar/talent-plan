pub mod traits;

pub use traits::{ProjectRepository, TaskRepository};

use crate::models::*;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

/// In-memory task repository for development and testing.
pub struct InMemoryTaskRepository {
    tasks: Mutex<HashMap<Uuid, Task>>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryTaskRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn create(&self, params: CreateTaskParams) -> Result<Task, RepositoryError> {
        let now = chrono::Utc::now();
        let task = Task {
            id: Uuid::new_v4(),
            project_id: params.project_id,
            title: params.title,
            description: params.description,
            status: TaskStatus::Todo,
            priority: params.priority,
            created_at: now,
            updated_at: now,
        };
        self.tasks.lock().unwrap().insert(task.id, task.clone());
        Ok(task)
    }

    async fn get(&self, id: Uuid) -> Result<Task, RepositoryError> {
        self.tasks
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(RepositoryError::NotFound)
    }

    async fn list(
        &self,
        filter: TaskFilter,
        pagination: PaginationParams,
    ) -> Result<PaginatedResponse<Task>, RepositoryError> {
        let tasks = self.tasks.lock().unwrap();
        let mut filtered: Vec<Task> = tasks
            .values()
            .filter(|t| {
                if let Some(ref pid) = filter.project_id {
                    if t.project_id != *pid {
                        return false;
                    }
                }
                if let Some(ref status) = filter.status {
                    if t.status != *status {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let total = filtered.len() as i64;
        let start = ((pagination.page - 1) * pagination.per_page) as usize;
        let items: Vec<Task> = filtered.into_iter().skip(start).take(pagination.per_page as usize).collect();

        Ok(PaginatedResponse {
            items,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
        })
    }

    async fn update(&self, id: Uuid, params: UpdateTaskParams) -> Result<Task, RepositoryError> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.get_mut(&id).ok_or(RepositoryError::NotFound)?;

        if let Some(title) = params.title {
            task.title = title;
        }
        if let Some(description) = params.description {
            task.description = description;
        }
        if let Some(status) = params.status {
            task.status = status;
        }
        if let Some(priority) = params.priority {
            task.priority = priority;
        }
        task.updated_at = chrono::Utc::now();

        Ok(task.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let removed = self.tasks.lock().unwrap().remove(&id);
        match removed {
            Some(_) => Ok(true),
            None => Err(RepositoryError::NotFound),
        }
    }
}

/// In-memory project repository for development and testing.
pub struct InMemoryProjectRepository {
    projects: Mutex<HashMap<Uuid, Project>>,
}

impl InMemoryProjectRepository {
    pub fn new() -> Self {
        Self {
            projects: Mutex::new(HashMap::new()),
        }
    }

    /// Seed a project into the repository (useful for tests).
    pub fn seed(&self, project: Project) {
        self.projects.lock().unwrap().insert(project.id, project);
    }
}

impl Default for InMemoryProjectRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ProjectRepository for InMemoryProjectRepository {
    async fn get(&self, id: Uuid) -> Result<Project, RepositoryError> {
        self.projects
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(RepositoryError::NotFound)
    }

    async fn get_many(&self, ids: Vec<Uuid>) -> Result<HashMap<Uuid, Project>, RepositoryError> {
        let projects = self.projects.lock().unwrap();
        let mut result = HashMap::new();
        for id in ids {
            if let Some(p) = projects.get(&id) {
                result.insert(id, p.clone());
            }
        }
        Ok(result)
    }
}

/// Repository error type.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("internal error: {0}")]
    Internal(String),
}
