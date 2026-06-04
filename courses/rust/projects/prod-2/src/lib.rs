pub mod errors;
pub mod models;
pub mod repository;
pub mod store;

// Re-export key types for convenience.
pub use errors::TaskError;
pub use models::*;
pub use repository::Repository;
pub use store::InMemoryTaskStore;

/// A service layer that wraps a Repository implementation.
///
/// This is the primary entry point for application logic. Students should
/// test this service both with mock repositories and with the real
/// InMemoryTaskStore.
pub struct TaskService<R: Repository> {
    repo: R,
}

impl<R: Repository> TaskService<R> {
    /// Create a new TaskService backed by the given repository.
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Create a new task.
    pub fn create_task(&mut self, request: CreateTask) -> Result<Task, TaskError> {
        self.repo.create(request)
    }

    /// Get a task by ID, returning an error if not found.
    pub fn get_task(&self, id: uuid::Uuid) -> Result<Task, TaskError> {
        self.repo
            .get(id)?
            .ok_or(TaskError::NotFound(id))
    }

    /// List tasks with optional filters and pagination.
    pub fn list_tasks(&self, params: ListParams) -> Result<Vec<Task>, TaskError> {
        self.repo.list(params)
    }

    /// Update a task by ID.
    pub fn update_task(
        &mut self,
        id: uuid::Uuid,
        request: UpdateTask,
    ) -> Result<Task, TaskError> {
        // Verify the task exists first.
        let _ = self.get_task(id)?;
        self.repo.update(id, request)
    }

    /// Delete a task by ID.
    pub fn delete_task(&mut self, id: uuid::Uuid) -> Result<(), TaskError> {
        // Verify the task exists first.
        let _ = self.get_task(id)?;
        self.repo.delete(id)
    }

    /// Transition a task's status.
    pub fn transition_status(
        &mut self,
        id: uuid::Uuid,
        new_status: TaskStatus,
    ) -> Result<Task, TaskError> {
        self.repo.transition_status(id, new_status)
    }

    /// Get the total number of tasks.
    pub fn count(&self) -> Result<usize, TaskError> {
        self.repo.count()
    }

    /// List tasks filtered by status.
    pub fn list_by_status(&self, status: TaskStatus) -> Result<Vec<Task>, TaskError> {
        self.repo.list(ListParams {
            status: Some(status),
            ..Default::default()
        })
    }
}
