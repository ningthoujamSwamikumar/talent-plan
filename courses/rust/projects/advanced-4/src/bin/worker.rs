//! Worker binary — starts a background job worker.
//!
//! Usage:
//!   DATABASE_URL=postgres://... cargo run --bin worker
//!
//! The worker connects to PostgreSQL, runs migrations, registers handlers,
//! and polls for jobs until it receives a shutdown signal (Ctrl-C).

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let worker_id = format!("worker-{}", uuid::Uuid::new_v4());
    let mut worker = job_processor::Worker::new(pool, worker_id);

    // TODO: Register handlers for your queues here, e.g.:
    // worker.register_handler("email", Box::new(EmailHandler));

    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(());

    // Listen for Ctrl-C
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl-c");
        tracing::info!("shutdown signal received");
        let _ = shutdown_tx.send(());
    });

    tracing::info!("worker starting");
    worker.run(shutdown_rx).await?;
    tracing::info!("worker stopped");

    Ok(())
}
