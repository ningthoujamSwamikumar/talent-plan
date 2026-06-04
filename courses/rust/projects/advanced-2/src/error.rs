use std::fmt;
use thiserror::Error;

/// Errors that can occur during cache operations.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("connection error: {0}")]
    Connection(String),

    #[error("operation timed out")]
    Timeout,

    #[error("internal cache error: {0}")]
    Internal(String),
}

/// Errors that can occur during rate limiting.
#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("rate limit exceeded")]
    LimitExceeded,

    #[error("internal rate limiter error: {0}")]
    Internal(String),
}

/// Errors that can occur during circuit breaker operations.
///
/// The type parameter `E` represents the underlying service error.
#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    /// The circuit is open and rejecting requests.
    Open,
    /// The underlying service returned an error.
    ServiceError(E),
    /// An internal circuit breaker error.
    Internal(String),
}

impl<E: fmt::Display> fmt::Display for CircuitBreakerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitBreakerError::Open => write!(f, "circuit breaker is open"),
            CircuitBreakerError::ServiceError(e) => write!(f, "service error: {}", e),
            CircuitBreakerError::Internal(msg) => {
                write!(f, "internal circuit breaker error: {}", msg)
            }
        }
    }
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for CircuitBreakerError<E> {}
