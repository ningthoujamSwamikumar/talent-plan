use async_trait::async_trait;

use crate::error::ConsumerError;
use crate::events::Event;

/// Trait implemented by anything that can react to an event.
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &Event) -> Result<(), ConsumerError>;
}

// ---------------------------------------------------------------------------
// Concrete handlers
// ---------------------------------------------------------------------------

/// Logs every event via `tracing::info!`.
pub struct LogHandler;

#[async_trait]
impl EventHandler for LogHandler {
    async fn handle(&self, _event: &Event) -> Result<(), ConsumerError> {
        todo!()
    }
}

/// POSTs the serialized event to a configured webhook URL.
pub struct WebhookHandler {
    pub url: String,
}

impl WebhookHandler {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[async_trait]
impl EventHandler for WebhookHandler {
    async fn handle(&self, _event: &Event) -> Result<(), ConsumerError> {
        todo!()
    }
}

/// Sends an email notification (stub).
pub struct EmailHandler {
    pub smtp_host: String,
    pub from: String,
    pub to: Vec<String>,
}

impl EmailHandler {
    pub fn new(smtp_host: impl Into<String>, from: impl Into<String>, to: Vec<String>) -> Self {
        Self {
            smtp_host: smtp_host.into(),
            from: from.into(),
            to,
        }
    }
}

#[async_trait]
impl EventHandler for EmailHandler {
    async fn handle(&self, _event: &Event) -> Result<(), ConsumerError> {
        todo!()
    }
}

/// A composite handler that dispatches to multiple inner handlers.
pub struct CompositeHandler {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl CompositeHandler {
    pub fn new(handlers: Vec<Box<dyn EventHandler>>) -> Self {
        Self { handlers }
    }
}

#[async_trait]
impl EventHandler for CompositeHandler {
    async fn handle(&self, _event: &Event) -> Result<(), ConsumerError> {
        todo!()
    }
}
