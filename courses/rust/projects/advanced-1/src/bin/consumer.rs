use std::sync::Arc;

use tokio::sync::watch;
use tracing_subscriber;

use event_consumer::{Consumer, ConsumerConfig, InMemoryBackend, LogHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // For demonstration purposes, use the in-memory backend.
    // Replace with `RedisBackend::connect("redis://127.0.0.1/").await?` for
    // production use.
    let backend = Arc::new(InMemoryBackend::new());
    let handler = Arc::new(LogHandler);

    let config = ConsumerConfig::new("tasks", "notification-service", "worker-1");
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let consumer = Consumer::new(backend, handler, config, shutdown_rx);

    // Shut down on Ctrl+C.
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl-c");
        let _ = shutdown_tx.send(true);
    });

    consumer.run().await?;

    Ok(())
}
