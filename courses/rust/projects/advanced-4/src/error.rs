use thiserror::Error;

/// Errors returned by the job processing system.
#[derive(Debug, Error)]
pub enum JobError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("job not found: {0}")]
    NotFound(uuid::Uuid),

    #[error("no handler registered for queue: {0}")]
    NoHandler(String),

    #[error("job already completed: {0}")]
    AlreadyCompleted(uuid::Uuid),

    #[error("job cancelled: {0}")]
    Cancelled(uuid::Uuid),

    #[error("invalid cron expression: {0}")]
    InvalidCron(String),

    #[error("handler error: {0}")]
    Handler(String),
}

/// Convenience Result alias for this crate.
pub type Result<T> = std::result::Result<T, JobError>;
