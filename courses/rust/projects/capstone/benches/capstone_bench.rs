//! Performance benchmarks for TaskForge Capstone.
//!
//! These benchmarks measure the latency and throughput of key operations.
//! They require a running server and database.
//!
//! Run with:
//!   TEST_SERVER_ADDR=http://127.0.0.1:3000 cargo bench

use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::runtime::Runtime;

fn server_addr() -> String {
    std::env::var("TEST_SERVER_ADDR").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string())
}

/// Register a user and return (access_token, project_id).
async fn setup_bench_env(client: &Client, base: &str) -> (String, String) {
    let unique = uuid::Uuid::new_v4().to_string();
    let email = format!("bench-{}@test.com", &unique[..8]);
    let password = "BenchPassword123!";

    client
        .post(format!("{base}/api/auth/register"))
        .json(&json!({
            "email": email,
            "password": password,
            "display_name": "Bench User"
        }))
        .send()
        .await
        .unwrap();

    let login_resp: Value = client
        .post(format!("{base}/api/auth/login"))
        .json(&json!({"email": email, "password": password}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let token = login_resp["access_token"].as_str().unwrap().to_string();

    let project: Value = client
        .post(format!("{base}/api/projects"))
        .bearer_auth(&token)
        .json(&json!({"name": "Bench Project", "description": "For benchmarks"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let project_id = project["id"].as_str().unwrap().to_string();
    (token, project_id)
}

fn bench_health_check(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let base = server_addr();
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    c.bench_function("health_live", |b| {
        b.to_async(&rt).iter(|| async {
            let resp = client
                .get(format!("{base}/health/live"))
                .send()
                .await
                .unwrap();
            assert!(resp.status().is_success());
        });
    });
}

fn bench_project_read(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let base = server_addr();
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let (token, project_id) = rt.block_on(setup_bench_env(&client, &base));

    c.bench_function("get_project", |b| {
        b.to_async(&rt).iter(|| {
            let client = client.clone();
            let base = base.clone();
            let token = token.clone();
            let pid = project_id.clone();
            async move {
                let resp = client
                    .get(format!("{base}/api/projects/{pid}"))
                    .bearer_auth(&token)
                    .send()
                    .await
                    .unwrap();
                assert!(resp.status().is_success());
            }
        });
    });
}

fn bench_task_create(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let base = server_addr();
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let (token, project_id) = rt.block_on(setup_bench_env(&client, &base));

    c.bench_function("create_task", |b| {
        b.to_async(&rt).iter(|| {
            let client = client.clone();
            let base = base.clone();
            let token = token.clone();
            let pid = project_id.clone();
            async move {
                let resp = client
                    .post(format!("{base}/api/projects/{pid}/tasks"))
                    .bearer_auth(&token)
                    .json(&json!({
                        "title": "Bench Task",
                        "priority": "medium"
                    }))
                    .send()
                    .await
                    .unwrap();
                assert!(resp.status().is_success());
            }
        });
    });
}

fn bench_task_list(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let base = server_addr();
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let (token, project_id) = rt.block_on(setup_bench_env(&client, &base));

    // Seed some tasks
    rt.block_on(async {
        for i in 0..20 {
            client
                .post(format!("{base}/api/projects/{project_id}/tasks"))
                .bearer_auth(&token)
                .json(&json!({"title": format!("Seed Task {i}"), "priority": "low"}))
                .send()
                .await
                .unwrap();
        }
    });

    c.bench_function("list_tasks", |b| {
        b.to_async(&rt).iter(|| {
            let client = client.clone();
            let base = base.clone();
            let token = token.clone();
            let pid = project_id.clone();
            async move {
                let resp = client
                    .get(format!("{base}/api/projects/{pid}/tasks?page=1&per_page=10"))
                    .bearer_auth(&token)
                    .send()
                    .await
                    .unwrap();
                assert!(resp.status().is_success());
            }
        });
    });
}

criterion_group!(
    benches,
    bench_health_check,
    bench_project_read,
    bench_task_create,
    bench_task_list,
);
criterion_main!(benches);
