use async_trait::async_trait;

use crate::error::BackendError;
use crate::events::Event;

/// Abstraction over a message queue / stream backend.
///
/// Implementations must be safe to share across tasks (`Send + Sync`).
#[async_trait]
pub trait MessageBackend: Send + Sync {
    /// Publish an event to the named stream.
    ///
    /// Returns the backend-assigned message ID on success.
    async fn publish(&self, stream: &str, event: &Event) -> Result<String, BackendError>;

    /// Read the next batch of unacknowledged messages for the given consumer
    /// group and consumer name.
    ///
    /// Returns a vec of `(message_id, event)` pairs. An empty vec means there
    /// are currently no pending messages.
    async fn subscribe(
        &self,
        stream: &str,
        group: &str,
        consumer: &str,
    ) -> Result<Vec<(String, Event)>, BackendError>;

    /// Acknowledge that a message has been successfully processed.
    async fn acknowledge(
        &self,
        stream: &str,
        group: &str,
        id: &str,
    ) -> Result<(), BackendError>;

    /// Move a poison message to the dead-letter stream for later inspection.
    async fn dead_letter(
        &self,
        stream: &str,
        id: &str,
        event: &Event,
        error: &str,
    ) -> Result<(), BackendError>;
}
