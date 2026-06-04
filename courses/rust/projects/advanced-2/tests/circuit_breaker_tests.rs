use std::time::Duration;

use resilience_middleware::circuit_breaker::{CircuitBreaker, CircuitState};
use resilience_middleware::error::CircuitBreakerError;

#[test]
fn circuit_breaker_starts_closed() {
    let cb = CircuitBreaker::new(3, Duration::from_secs(1));
    assert_eq!(cb.state(), CircuitState::Closed);
}

#[test]
fn circuit_breaker_stays_closed_below_threshold() {
    let cb = CircuitBreaker::new(3, Duration::from_secs(1));
    cb.record_failure();
    cb.record_failure();
    // Two failures, threshold is 3 — still closed.
    assert_eq!(cb.state(), CircuitState::Closed);
}

#[test]
fn circuit_breaker_opens_after_threshold_failures() {
    let cb = CircuitBreaker::new(3, Duration::from_secs(1));
    cb.record_failure();
    cb.record_failure();
    cb.record_failure();
    assert_eq!(cb.state(), CircuitState::Open);
}

#[tokio::test]
async fn circuit_breaker_rejects_calls_when_open() {
    let cb = CircuitBreaker::new(1, Duration::from_secs(60));
    cb.record_failure(); // Opens the circuit.

    let result: Result<(), CircuitBreakerError<&str>> =
        cb.call(async { Ok::<(), &str>(()) }).await;

    assert!(matches!(result, Err(CircuitBreakerError::Open)));
}

#[tokio::test]
async fn circuit_breaker_transitions_to_half_open_after_timeout() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(50));
    cb.record_failure();
    assert_eq!(cb.state(), CircuitState::Open);

    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(cb.state(), CircuitState::HalfOpen);
}

#[tokio::test]
async fn circuit_breaker_closes_on_successful_probe() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(50));
    cb.record_failure();
    assert_eq!(cb.state(), CircuitState::Open);

    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(cb.state(), CircuitState::HalfOpen);

    // Successful probe should close the circuit.
    let result: Result<&str, CircuitBreakerError<&str>> =
        cb.call(async { Ok::<&str, &str>("ok") }).await;
    assert!(result.is_ok());
    assert_eq!(cb.state(), CircuitState::Closed);
}

#[tokio::test]
async fn circuit_breaker_reopens_on_failed_probe() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(50));
    cb.record_failure();

    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(cb.state(), CircuitState::HalfOpen);

    // Failed probe should reopen the circuit.
    let result: Result<(), CircuitBreakerError<&str>> =
        cb.call(async { Err::<(), &str>("boom") }).await;
    assert!(matches!(result, Err(CircuitBreakerError::ServiceError("boom"))));
    assert_eq!(cb.state(), CircuitState::Open);
}

#[test]
fn circuit_breaker_success_resets_failure_count() {
    let cb = CircuitBreaker::new(3, Duration::from_secs(1));
    cb.record_failure();
    cb.record_failure();
    cb.record_success(); // Resets count.
    cb.record_failure();
    cb.record_failure();
    // Only 2 failures since last success — should still be closed.
    assert_eq!(cb.state(), CircuitState::Closed);
}
