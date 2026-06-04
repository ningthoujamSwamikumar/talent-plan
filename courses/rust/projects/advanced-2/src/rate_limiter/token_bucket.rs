use std::sync::Mutex;
use std::time::Instant;

/// A token bucket rate limiter.
///
/// Tokens are consumed on each request and refilled at a constant rate.
/// The bucket never holds more tokens than its capacity. Refilling is
/// computed lazily at access time rather than via a background timer.
pub struct TokenBucket {
    _inner: Mutex<TokenBucketInner>,
}

struct TokenBucketInner {
    /// Maximum number of tokens the bucket can hold.
    _capacity: u32,
    /// Current number of available tokens (as f64 for fractional refill).
    _tokens: f64,
    /// Tokens added per second.
    _refill_rate: f64,
    /// The last time tokens were refilled.
    _last_refill: Instant,
}

impl TokenBucket {
    /// Create a new token bucket.
    ///
    /// # Arguments
    /// - `capacity` — the maximum number of tokens (also the initial count).
    /// - `refill_rate` — tokens added per second.
    pub fn new(_capacity: u32, _refill_rate: f64) -> Self {
        // TODO: Initialize with full capacity and record the current time.
        todo!()
    }

    /// Try to consume one token. Returns `true` if a token was available.
    pub fn try_acquire(&self) -> bool {
        // TODO: Refill based on elapsed time, then try to consume 1 token.
        todo!()
    }

    /// Try to consume `n` tokens at once. Returns `true` if enough tokens
    /// were available.
    pub fn try_acquire_n(&self, _n: u32) -> bool {
        // TODO: Refill based on elapsed time, then try to consume n tokens.
        todo!()
    }

    /// Return the number of tokens currently available (after refilling).
    pub fn remaining(&self) -> u32 {
        // TODO: Refill and return the token count (truncated to u32).
        todo!()
    }
}
