use crate::models::TaskStatus;
use uuid::Uuid;

/// Errors that can occur in the task management system.
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Task not found: {0}")]
    NotFound(Uuid),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid status transition from {from} to {to}")]
    InvalidTransition { from: TaskStatus, to: TaskStatus },

    #[error("Storage error: {0}")]
    StorageError(String),
}
