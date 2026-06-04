use thiserror::Error;

/// Errors originating from a message backend implementation.
#[derive(Debug, Error)]
pub enum BackendError {
    #[error("connection error: {0}")]
    Connection(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("stream not found: {0}")]
    StreamNotFound(String),

    #[error("consumer group error: {0}")]
    ConsumerGroup(String),

    #[error("internal error: {0}")]
    Internal(String),
}

/// Errors that may occur while consuming or handling events.
#[derive(Debug, Error)]
pub enum ConsumerError {
    #[error("backend error: {0}")]
    Backend(#[from] BackendError),

    #[error("handler error: {0}")]
    Handler(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),

    #[error("shutdown requested")]
    Shutdown,

    #[error("max retries exceeded for message {id}: {reason}")]
    MaxRetriesExceeded { id: String, reason: String },
}
