use std::sync::Arc;

use tokio::sync::watch;

use crate::backend::MessageBackend;
use crate::error::ConsumerError;
use crate::handler::EventHandler;

/// Configuration for the consumer worker.
pub struct ConsumerConfig {
    /// The stream to consume from.
    pub stream: String,
    /// The consumer group name.
    pub group: String,
    /// This consumer's unique name within the group.
    pub consumer_name: String,
    /// Maximum number of retries before dead-lettering a message.
    pub max_retries: u32,
}

impl ConsumerConfig {
    pub fn new(
        stream: impl Into<String>,
        group: impl Into<String>,
        consumer_name: impl Into<String>,
    ) -> Self {
        Self {
            stream: stream.into(),
            group: group.into(),
            consumer_name: consumer_name.into(),
            max_retries: 3,
        }
    }

    pub fn with_max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
}

/// The consumer worker: reads events from a backend, dispatches them to a
/// handler, acknowledges on success, and dead-letters after max retries.
pub struct Consumer {
    _backend: Arc<dyn MessageBackend>,
    _handler: Arc<dyn EventHandler>,
    _config: ConsumerConfig,
    _shutdown_rx: watch::Receiver<bool>,
}

impl Consumer {
    /// Create a new consumer.
    ///
    /// - `backend` -- the message queue backend to read from.
    /// - `handler` -- the event handler to dispatch events to.
    /// - `config` -- consumer configuration (stream, group, name, retries).
    /// - `shutdown_rx` -- when this watch channel receives `true`, the consumer
    ///   finishes its current batch and exits.
    pub fn new(
        backend: Arc<dyn MessageBackend>,
        handler: Arc<dyn EventHandler>,
        config: ConsumerConfig,
        shutdown_rx: watch::Receiver<bool>,
    ) -> Self {
        Self {
            _backend: backend,
            _handler: handler,
            _config: config,
            _shutdown_rx: shutdown_rx,
        }
    }

    /// Run the consumer loop until shutdown is signalled.
    ///
    /// The loop should:
    /// 1. Check if shutdown has been requested; if so, return `Ok(())`.
    /// 2. Call `backend.subscribe(...)` to fetch the next batch of messages.
    /// 3. For each message, call `handler.handle(event)`.
    ///    - On success, acknowledge the message.
    ///    - On failure, retry with exponential backoff up to `max_retries`.
    ///    - After exhausting retries, dead-letter the message.
    /// 4. Go to step 1.
    pub async fn run(&self) -> Result<(), ConsumerError> {
        todo!()
    }
}
