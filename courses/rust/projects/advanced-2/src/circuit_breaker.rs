use std::future::Future;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::error::CircuitBreakerError;

/// The state of a circuit breaker.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    /// Requests pass through normally. Failures are counted.
    Closed,
    /// Requests are immediately rejected.
    Open,
    /// A single probe request is allowed through to test recovery.
    HalfOpen,
}

/// A circuit breaker that protects calls to an external service.
///
/// When failures exceed a threshold the circuit opens, rejecting all
/// requests until a timeout elapses and a probe is attempted.
pub struct CircuitBreaker {
    _inner: Mutex<CircuitBreakerInner>,
}

struct CircuitBreakerInner {
    /// Current state of the circuit.
    _state: CircuitState,
    /// Number of consecutive failures.
    _failure_count: u32,
    /// Number of failures before the circuit opens.
    _failure_threshold: u32,
    /// How long the circuit stays open before transitioning to half-open.
    _timeout: Duration,
    /// When the last failure occurred (used for timeout calculation).
    _last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    ///
    /// # Arguments
    /// - `failure_threshold` — how many failures trigger the open state.
    /// - `timeout` — how long to wait in the open state before probing.
    pub fn new(_failure_threshold: u32, _timeout: Duration) -> Self {
        // TODO: Initialize in the Closed state with zero failures.
        todo!()
    }

    /// Return the current state of the circuit breaker.
    ///
    /// This takes into account whether the timeout has elapsed (i.e., an
    /// Open breaker whose timeout has passed should report HalfOpen).
    pub fn state(&self) -> CircuitState {
        // TODO: Lock inner, check if Open should transition to HalfOpen.
        todo!()
    }

    /// Execute an async operation through the circuit breaker.
    ///
    /// - If the circuit is **Open**, the call is rejected immediately.
    /// - If the circuit is **Closed** or **HalfOpen**, the future is awaited.
    /// - On success, `record_success` is called.
    /// - On failure, `record_failure` is called.
    pub async fn call<F, T, E>(&self, _f: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: Future<Output = Result<T, E>>,
    {
        // TODO: Check state. If Open, return Err(CircuitBreakerError::Open).
        // TODO: Otherwise await the future.
        // TODO: Record success or failure based on the result.
        todo!()
    }

    /// Record a successful call, resetting the failure count and closing
    /// the circuit if it was half-open.
    pub fn record_success(&self) {
        // TODO: Reset failure count to 0. Transition to Closed if HalfOpen.
        todo!()
    }

    /// Record a failed call, incrementing the failure count and
    /// potentially opening the circuit.
    pub fn record_failure(&self) {
        // TODO: Increment failure count.
        // TODO: If count >= threshold, transition to Open and record time.
        todo!()
    }
}
