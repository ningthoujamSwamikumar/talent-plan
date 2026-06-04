use std::sync::Arc;

use axum::{
    extract::State,
    routing::get,
    Router,
};
use tokio::sync::broadcast;
use tonic::transport::Server as TonicServer;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

use taskforge_multi::grpc::proto::task_service_server::TaskServiceServer;
use taskforge_multi::grpc::TaskServiceImpl;
use taskforge_multi::graphql::{build_schema, AppSchema};
use taskforge_multi::models::TaskChangeEvent;
use taskforge_multi::repository::{InMemoryProjectRepository, InMemoryTaskRepository};
use taskforge_multi::AppState;

/// GraphQL handler: execute queries and mutations.
async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphQL playground (GET /graphql).
async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(
        async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
                .subscription_endpoint("/graphql/ws"),
        ),
    )
}

#[tokio::main]
async fn main() {
    // Initialize tracing.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Shared infrastructure.
    let task_repo: Arc<dyn taskforge_multi::repository::TaskRepository> =
        Arc::new(InMemoryTaskRepository::new());
    let project_repo: Arc<dyn taskforge_multi::repository::ProjectRepository> =
        Arc::new(InMemoryProjectRepository::new());
    let (event_tx, _) = broadcast::channel::<TaskChangeEvent>(256);

    let _state = AppState {
        task_repo: task_repo.clone(),
        project_repo: project_repo.clone(),
        event_tx: event_tx.clone(),
    };

    // --- GraphQL schema ---
    let schema = build_schema(task_repo.clone(), event_tx.clone());

    // --- Axum (REST + GraphQL) server ---
    // NOTE: GraphQL subscriptions over WebSocket require matching axum versions
    // between async-graphql-axum and the main router. In production, wire up
    // `GraphQLSubscription::new(schema)` on the "/graphql/ws" route once versions
    // are aligned. For now, queries and mutations are served on "/graphql".
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/health", get(|| async { "OK" }))
        .with_state(schema)
        .layer(CorsLayer::permissive());

    let axum_addr = "0.0.0.0:3000";
    let grpc_addr = "0.0.0.0:50051".parse().unwrap();

    tracing::info!("Starting REST + GraphQL server on {}", axum_addr);
    tracing::info!("Starting gRPC server on {}", grpc_addr);

    // --- gRPC server ---
    let grpc_service = TaskServiceImpl::new(task_repo.clone(), event_tx.clone());

    let grpc_handle = tokio::spawn(async move {
        TonicServer::builder()
            .add_service(TaskServiceServer::new(grpc_service))
            .serve(grpc_addr)
            .await
            .expect("gRPC server failed");
    });

    let axum_listener = tokio::net::TcpListener::bind(axum_addr).await.unwrap();

    let axum_handle = tokio::spawn(async move {
        axum::serve(axum_listener, app).await.expect("Axum server failed");
    });

    // Wait for Ctrl+C, then shut down.
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for Ctrl+C");

    tracing::info!("Shutting down...");
    grpc_handle.abort();
    axum_handle.abort();
}
