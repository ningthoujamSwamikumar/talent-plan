//! CLI tool to enqueue test jobs.
//!
//! Usage:
//!   DATABASE_URL=postgres://... cargo run --bin enqueue -- <queue> [payload-json]
//!
//! Examples:
//!   cargo run --bin enqueue -- email '{"to":"user@example.com","subject":"Hello"}'
//!   cargo run --bin enqueue -- reports '{"report_id":42}'

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: enqueue <queue> [payload-json]");
        std::process::exit(1);
    }

    let queue = &args[1];
    let payload: serde_json::Value = if args.len() >= 3 {
        serde_json::from_str(&args[2])?
    } else {
        serde_json::json!({})
    };

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let client = job_processor::JobClient::new(pool);
    let job_id = client.enqueue(queue, payload.clone()).await?;

    println!("Enqueued job {} on queue '{}' with payload: {}", job_id, queue, payload);

    Ok(())
}
