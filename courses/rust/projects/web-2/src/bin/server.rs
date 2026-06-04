use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use taskforge_db::handlers::{project_routes, task_routes};
use taskforge_db::repository::{ProjectRepository, TaskRepository};

#[tokio::main]
async fn main() {
    // Load .env file (silently ignore if missing)
    dotenvy::dotenv().ok();

    // Initialise structured logging
    tracing_subscriber::fmt::init();

    // Read database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool with sensible defaults
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Run embedded migrations (from the ./migrations directory)
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    info!("Migrations applied successfully");

    // Build repositories
    let _project_repo = ProjectRepository::new(pool.clone());
    let _task_repo = TaskRepository::new(pool.clone());

    // Build the axum application
    //
    // NOTE: Because ProjectRepository and TaskRepository are separate State
    // types, you will need to either:
    //   (a) wrap both in a single shared AppState struct, or
    //   (b) merge two routers each with their own state.
    // The stubs below show approach (b) -- students should complete this.
    let _app = axum::Router::new()
        // TODO: merge project_routes().with_state(project_repo)
        //       and   task_routes().with_state(task_repo)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, _app).await.expect("Server error");
}
