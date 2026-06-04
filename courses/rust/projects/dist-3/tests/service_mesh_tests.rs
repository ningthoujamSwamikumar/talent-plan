use chrono::Utc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use service_mesh::idempotency::{CachedResponse, IdempotencyStore};
use service_mesh::load_balancer::{LeastConnections, LoadBalancer, Random, RoundRobin};
use service_mesh::registry::{ServiceInstance, ServiceRegistry};
use service_mesh::saga::{Saga, SagaOrchestrator, SagaStep};
// SpanCollector is available for use in custom tracing tests.
#[allow(unused_imports)]
use service_mesh::tracing_propagation::SpanCollector;

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

fn make_instance(id: &str, name: &str) -> ServiceInstance {
    ServiceInstance {
        id: id.to_string(),
        service_name: name.to_string(),
        address: format!("http://127.0.0.1:{}", 3000 + id.len()),
        last_heartbeat: Utc::now(),
    }
}

fn make_stale_instance(id: &str, name: &str) -> ServiceInstance {
    ServiceInstance {
        id: id.to_string(),
        service_name: name.to_string(),
        address: format!("http://127.0.0.1:{}", 3000 + id.len()),
        last_heartbeat: Utc::now() - chrono::Duration::seconds(120),
    }
}

// -------------------------------------------------------------------------
// Registry tests
// -------------------------------------------------------------------------

#[test]
fn registry_register_and_discover() {
    let registry = ServiceRegistry::new(Duration::from_secs(30));
    registry.register(make_instance("a1", "svc-a"));
    registry.register(make_instance("a2", "svc-a"));
    registry.register(make_instance("b1", "svc-b"));

    let discovered = registry.discover("svc-a");
    assert_eq!(discovered.len(), 2);
    assert!(discovered.iter().all(|i| i.service_name == "svc-a"));
}

#[test]
fn registry_deregister_removes_service() {
    let registry = ServiceRegistry::new(Duration::from_secs(30));
    registry.register(make_instance("a1", "svc-a"));
    registry.register(make_instance("a2", "svc-a"));

    registry.deregister("a1");

    let discovered = registry.discover("svc-a");
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].id, "a2");
}

#[test]
fn registry_health_check_removes_unhealthy() {
    let registry = ServiceRegistry::new(Duration::from_secs(30));
    registry.register(make_instance("healthy", "svc"));
    registry.register(make_stale_instance("stale", "svc"));

    registry.health_check();

    let discovered = registry.discover("svc");
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].id, "healthy");
}

// -------------------------------------------------------------------------
// Load balancer tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn round_robin_distributes_evenly() {
    let lb = RoundRobin::new();
    let instances = vec![
        make_instance("r1", "svc"),
        make_instance("r2", "svc"),
        make_instance("r3", "svc"),
    ];

    let mut counts = [0usize; 3];
    for _ in 0..9 {
        let inst = lb.next_instance(&instances).await.unwrap();
        if inst.id == "r1" {
            counts[0] += 1;
        } else if inst.id == "r2" {
            counts[1] += 1;
        } else {
            counts[2] += 1;
        }
    }
    // Each instance should be picked exactly 3 times.
    assert_eq!(counts, [3, 3, 3]);
}

#[tokio::test]
async fn random_distributes_to_all() {
    let lb = Random::new();
    let instances = vec![
        make_instance("r1", "svc"),
        make_instance("r2", "svc"),
    ];

    let mut seen = std::collections::HashSet::new();
    // Run enough iterations that both are almost certainly seen.
    for _ in 0..100 {
        let inst = lb.next_instance(&instances).await.unwrap();
        seen.insert(inst.id.clone());
    }
    assert!(seen.contains("r1"), "r1 was never selected");
    assert!(seen.contains("r2"), "r2 was never selected");
}

#[tokio::test]
async fn least_connections_prefers_idle() {
    let lb = LeastConnections::new();
    lb.connect("busy");
    lb.connect("busy");
    lb.connect("busy");

    let instances = vec![
        make_instance("busy", "svc"),
        make_instance("idle", "svc"),
    ];

    let picked = lb.next_instance(&instances).await.unwrap();
    assert_eq!(picked.id, "idle");
}

// -------------------------------------------------------------------------
// Tracing tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn trace_id_propagated_across_services() {
    // This test verifies that a trace ID set on an incoming request is echoed
    // back on the response. It requires a running Axum handler wired with the
    // trace_id_middleware.
    use axum::{routing::get, Router};
    use service_mesh::tracing_propagation::trace_id_middleware;

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .layer(axum::middleware::from_fn(trace_id_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("http://{}/ping", addr))
        .header("x-trace-id", "test-trace-123")
        .send()
        .await
        .unwrap();

    let returned_trace = resp
        .headers()
        .get("x-trace-id")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(returned_trace, "test-trace-123");
}

#[tokio::test]
async fn trace_id_generated_if_missing() {
    use axum::{routing::get, Router};
    use service_mesh::tracing_propagation::trace_id_middleware;

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .layer(axum::middleware::from_fn(trace_id_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("http://{}/ping", addr))
        // No x-trace-id header sent
        .send()
        .await
        .unwrap();

    let trace_header = resp.headers().get("x-trace-id");
    assert!(trace_header.is_some(), "trace ID should be generated");
    let value = trace_header.unwrap().to_str().unwrap();
    assert!(!value.is_empty());
    // Should be a valid UUID
    assert!(uuid::Uuid::parse_str(value).is_ok(), "generated trace ID should be a UUID");
}

// -------------------------------------------------------------------------
// Saga tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn saga_completes_all_steps() {
    let executed = Arc::new(AtomicUsize::new(0));

    let mut saga = Saga::new();
    for _ in 0..3 {
        let exec = executed.clone();
        saga.add_step(SagaStep {
            action: Box::new(move || {
                let exec = exec.clone();
                Box::pin(async move {
                    exec.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
            }),
            compensation: Box::new(|| Box::pin(async { Ok(()) })),
        });
    }

    let result = SagaOrchestrator::execute(&saga).await;
    assert!(result.is_ok());
    assert_eq!(executed.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn saga_compensates_on_failure() {
    // Step 3 (index 2) fails; steps 1 and 0 should be compensated.
    let compensated = Arc::new(std::sync::Mutex::new(Vec::<usize>::new()));

    let mut saga = Saga::new();
    for i in 0..3 {
        let comp = compensated.clone();
        let action: Box<dyn Fn() -> service_mesh::saga::BoxFuture<'static, Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync> = if i == 2 {
            Box::new(|| {
                Box::pin(async {
                    Err(Box::<dyn std::error::Error + Send + Sync>::from("step 2 failed"))
                })
            })
        } else {
            Box::new(|| Box::pin(async { Ok(()) }))
        };

        saga.add_step(SagaStep {
            action,
            compensation: Box::new(move || {
                let comp = comp.clone();
                Box::pin(async move {
                    comp.lock().unwrap().push(i);
                    Ok(())
                })
            }),
        });
    }

    let result = SagaOrchestrator::execute(&saga).await;
    assert!(result.is_err());

    let comp_order = compensated.lock().unwrap().clone();
    // Steps 1 and 0 should be compensated in reverse order.
    assert_eq!(comp_order, vec![1, 0]);
}

#[tokio::test]
async fn saga_partial_compensation() {
    // Step 2 (index 1) fails; only step 0 should be compensated.
    let compensated = Arc::new(std::sync::Mutex::new(Vec::<usize>::new()));

    let mut saga = Saga::new();
    for i in 0..3 {
        let comp = compensated.clone();
        let action: Box<dyn Fn() -> service_mesh::saga::BoxFuture<'static, Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync> = if i == 1 {
            Box::new(|| {
                Box::pin(async {
                    Err(Box::<dyn std::error::Error + Send + Sync>::from("step 1 failed"))
                })
            })
        } else {
            Box::new(|| Box::pin(async { Ok(()) }))
        };

        saga.add_step(SagaStep {
            action,
            compensation: Box::new(move || {
                let comp = comp.clone();
                Box::pin(async move {
                    comp.lock().unwrap().push(i);
                    Ok(())
                })
            }),
        });
    }

    let result = SagaOrchestrator::execute(&saga).await;
    assert!(result.is_err());

    let comp_order = compensated.lock().unwrap().clone();
    assert_eq!(comp_order, vec![0]);
}

// -------------------------------------------------------------------------
// Idempotency tests
// -------------------------------------------------------------------------

#[test]
fn idempotency_key_prevents_duplicate() {
    let store = IdempotencyStore::new();

    let response = CachedResponse {
        status: axum::http::StatusCode::OK,
        body: b"first response".to_vec(),
    };
    store.insert("key-1".to_string(), response.clone());

    let cached = store.get("key-1");
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().body, b"first response");
}

#[test]
fn idempotency_key_missing_is_allowed() {
    let store = IdempotencyStore::new();
    let cached = store.get("nonexistent");
    assert!(cached.is_none());
}

// -------------------------------------------------------------------------
// Integration test
// -------------------------------------------------------------------------

#[tokio::test]
async fn full_integration_create_task_saga() {
    // Simulates the create-task saga: three steps that represent calling
    // Service A (create task), Service B (notify), and Service C (index).
    let results = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));

    let mut saga = Saga::new();
    for (_i, name) in ["create_task", "notify", "index"].iter().enumerate() {
        let res = results.clone();
        let name = name.to_string();
        saga.add_step(SagaStep {
            action: Box::new(move || {
                let res = res.clone();
                let name = name.clone();
                Box::pin(async move {
                    res.lock().unwrap().push(name);
                    Ok(())
                })
            }),
            compensation: Box::new(|| Box::pin(async { Ok(()) })),
        });
    }

    let result = SagaOrchestrator::execute(&saga).await;
    assert!(result.is_ok());

    let actions = results.lock().unwrap().clone();
    assert_eq!(actions, vec!["create_task", "notify", "index"]);
}
