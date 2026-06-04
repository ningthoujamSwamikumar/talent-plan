use async_trait::async_trait;

/// Trait that job handlers must implement.
///
/// Each handler is associated with a queue name. When a job is dequeued from
/// that queue the worker calls [`JobHandler::handle`] with the job payload.
#[async_trait]
pub trait JobHandler: Send + Sync {
    /// Process a job payload. Return `Ok(())` on success or an error on failure.
    async fn handle(
        &self,
        payload: serde_json::Value,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// The queue name this handler is responsible for.
    fn queue(&self) -> &str;
}
