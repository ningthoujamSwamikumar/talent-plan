pub mod middleware;
pub mod token_bucket;

use async_trait::async_trait;

use crate::error::RateLimitError;

/// A trait for rate limiter implementations.
#[async_trait]
pub trait RateLimiter: Send + Sync {
    /// Try to acquire a single permit. Returns `Ok(())` if allowed,
    /// or `Err(RateLimitError::LimitExceeded)` if the rate limit has
    /// been reached.
    async fn try_acquire(&self) -> Result<(), RateLimitError>;

    /// Try to acquire `n` permits at once.
    async fn try_acquire_n(&self, n: u32) -> Result<(), RateLimitError>;

    /// Return the number of permits currently available.
    async fn remaining(&self) -> u32;
}
