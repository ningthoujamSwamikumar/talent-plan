use crate::errors::TaskError;
use crate::models::{CreateTask, ListParams, Task, TaskStatus, UpdateTask};
use uuid::Uuid;

/// The Repository trait defines the interface for task persistence.
///
/// Students should use `mockall` to create a mock implementation of this trait
/// for testing service logic in isolation.
pub trait Repository: Send + Sync {
    /// Create a new task from the given request. Returns the created task.
    fn create(&mut self, request: CreateTask) -> Result<Task, TaskError>;

    /// Get a task by its ID. Returns None if not found.
    fn get(&self, id: Uuid) -> Result<Option<Task>, TaskError>;

    /// List tasks matching the given parameters.
    fn list(&self, params: ListParams) -> Result<Vec<Task>, TaskError>;

    /// Update a task by its ID. Returns the updated task.
    fn update(&mut self, id: Uuid, request: UpdateTask) -> Result<Task, TaskError>;

    /// Delete a task by its ID.
    fn delete(&mut self, id: Uuid) -> Result<(), TaskError>;

    /// Transition a task's status.
    fn transition_status(
        &mut self,
        id: Uuid,
        new_status: TaskStatus,
    ) -> Result<Task, TaskError>;

    /// Return the total count of tasks in the store.
    fn count(&self) -> Result<usize, TaskError>;
}
