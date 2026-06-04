use std::future::Future;
use std::pin::Pin;
use thiserror::Error;

/// Errors that can occur during saga execution.
#[derive(Debug, Error)]
pub enum SagaError {
    #[error("step {step} failed: {source}")]
    StepFailed {
        step: usize,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("compensation for step {step} failed: {source}")]
    CompensationFailed {
        step: usize,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// A boxed future returned by saga action/compensation closures.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A single step in a saga: a forward action and its compensating rollback.
pub struct SagaStep {
    /// The forward action. Returns `Ok(())` on success.
    pub action: Box<dyn Fn() -> BoxFuture<'static, Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync>,
    /// The compensating action to undo the forward action.
    pub compensation: Box<dyn Fn() -> BoxFuture<'static, Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync>,
}

/// An ordered sequence of saga steps.
pub struct Saga {
    pub steps: Vec<SagaStep>,
}

impl Saga {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(&mut self, step: SagaStep) {
        self.steps.push(step);
    }
}

impl Default for Saga {
    fn default() -> Self {
        Self::new()
    }
}

/// Executes a saga, running compensations in reverse on failure.
pub struct SagaOrchestrator;

impl SagaOrchestrator {
    /// Execute all steps in the saga. If step N fails, compensate steps
    /// N-1 .. 0 in reverse order.
    pub async fn execute(saga: &Saga) -> Result<(), SagaError> {
        // TODO:
        // 1. Iterate through saga.steps, calling each action.
        // 2. On failure at step i, compensate steps i-1 down to 0.
        // 3. Return Ok(()) if all steps succeed.
        let _ = saga;
        todo!("SagaOrchestrator::execute")
    }
}
