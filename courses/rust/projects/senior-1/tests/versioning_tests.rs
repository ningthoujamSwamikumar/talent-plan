//! Integration tests for API versioning.
//!
//! These tests start the server on a random port and exercise the API using reqwest.
//! They do NOT import types from the crate — all assertions are based on JSON values,
//! so students can change internal types freely without breaking the test harness.

use reqwest::Client;
use serde_json::{json, Value};
use std::net::TcpListener;

/// Find an available port and start the server in the background.
/// Returns the base URL (e.g., "http://127.0.0.1:12345").
async fn start_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let addr = format!("127.0.0.1:{port}");
    let base_url = format!("http://{addr}");

    tokio::spawn(async move {
        api_design::run_server(&addr).await.unwrap();
    });

    // Give the server a moment to bind
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    base_url
}

fn client() -> Client {
    Client::new()
}

// =========================================================================
// Part 1 & 2: OpenAPI spec and Swagger UI
// =========================================================================

#[tokio::test]
async fn test_openapi_spec_is_valid_json() {
    let base = start_server().await;
    let resp = client()
        .get(format!("{base}/api-doc/openapi.json"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body.get("openapi").is_some(), "Missing 'openapi' field");
    assert!(body.get("paths").is_some(), "Missing 'paths' field");
}

#[tokio::test]
async fn test_openapi_spec_contains_v1_and_v2_paths() {
    let base = start_server().await;
    let body: Value = client()
        .get(format!("{base}/api-doc/openapi.json"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let paths = body.get("paths").unwrap();
    assert!(paths.get("/v1/tasks").is_some(), "Missing /v1/tasks path");
    assert!(paths.get("/v2/tasks").is_some(), "Missing /v2/tasks path");
}

#[tokio::test]
async fn test_swagger_ui_is_accessible() {
    let base = start_server().await;
    let resp = client()
        .get(format!("{base}/swagger-ui/"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let text = resp.text().await.unwrap();
    assert!(text.contains("swagger"), "Swagger UI page should contain 'swagger'");
}

// =========================================================================
// Part 3: V1 CRUD operations
// =========================================================================

#[tokio::test]
async fn test_v1_create_and_get_task() {
    let base = start_server().await;
    let c = client();

    let create_resp = c
        .post(format!("{base}/v1/tasks"))
        .json(&json!({
            "title": "Write tests",
            "description": "Unit and integration tests",
            "priority": "high"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(create_resp.status(), 201);
    let task: Value = create_resp.json().await.unwrap();
    let id = task["id"].as_str().unwrap();

    let get_resp = c.get(format!("{base}/v1/tasks/{id}")).send().await.unwrap();
    assert_eq!(get_resp.status(), 200);
    let fetched: Value = get_resp.json().await.unwrap();
    assert_eq!(fetched["title"], "Write tests");
    assert_eq!(fetched["priority"], "high");
    // v1 should NOT include tags or due_date
    assert!(fetched.get("tags").is_none(), "v1 response should not include tags");
    assert!(fetched.get("due_date").is_none(), "v1 response should not include due_date");
}

#[tokio::test]
async fn test_v1_list_tasks() {
    let base = start_server().await;
    let c = client();

    // Create two tasks
    c.post(format!("{base}/v1/tasks"))
        .json(&json!({"title": "Task A"}))
        .send()
        .await
        .unwrap();
    c.post(format!("{base}/v1/tasks"))
        .json(&json!({"title": "Task B"}))
        .send()
        .await
        .unwrap();

    let resp = c.get(format!("{base}/v1/tasks")).send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let tasks: Vec<Value> = resp.json().await.unwrap();
    assert!(tasks.len() >= 2);
}

#[tokio::test]
async fn test_v1_update_task() {
    let base = start_server().await;
    let c = client();

    let task: Value = c
        .post(format!("{base}/v1/tasks"))
        .json(&json!({"title": "Original"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let id = task["id"].as_str().unwrap();

    let updated: Value = c
        .put(format!("{base}/v1/tasks/{id}"))
        .json(&json!({"title": "Updated", "status": "in_progress"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(updated["title"], "Updated");
    assert_eq!(updated["status"], "in_progress");
}

#[tokio::test]
async fn test_v1_delete_task() {
    let base = start_server().await;
    let c = client();

    let task: Value = c
        .post(format!("{base}/v1/tasks"))
        .json(&json!({"title": "To delete"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let id = task["id"].as_str().unwrap();

    let del_resp = c.delete(format!("{base}/v1/tasks/{id}")).send().await.unwrap();
    assert_eq!(del_resp.status(), 204);

    let get_resp = c.get(format!("{base}/v1/tasks/{id}")).send().await.unwrap();
    assert_eq!(get_resp.status(), 404);
}

#[tokio::test]
async fn test_v1_get_nonexistent_returns_404() {
    let base = start_server().await;
    let resp = client()
        .get(format!("{base}/v1/tasks/00000000-0000-0000-0000-000000000000"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// =========================================================================
// Part 3: V2 CRUD operations
// =========================================================================

#[tokio::test]
async fn test_v2_create_with_tags_and_due_date() {
    let base = start_server().await;
    let c = client();

    let task: Value = c
        .post(format!("{base}/v2/tasks"))
        .json(&json!({
            "title": "V2 task",
            "tags": ["backend", "urgent"],
            "due_date": "2025-12-31T23:59:59Z"
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(task["title"], "V2 task");
    assert_eq!(task["tags"], json!(["backend", "urgent"]));
    assert!(task.get("due_date").is_some());
}

#[tokio::test]
async fn test_v2_update_tags() {
    let base = start_server().await;
    let c = client();

    let task: Value = c
        .post(format!("{base}/v2/tasks"))
        .json(&json!({"title": "Taggable", "tags": ["alpha"]}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let id = task["id"].as_str().unwrap();

    let updated: Value = c
        .put(format!("{base}/v2/tasks/{id}"))
        .json(&json!({"tags": ["alpha", "beta"]}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(updated["tags"], json!(["alpha", "beta"]));
}

// =========================================================================
// Part 4: Cross-version compatibility
// =========================================================================

#[tokio::test]
async fn test_v2_task_readable_via_v1() {
    let base = start_server().await;
    let c = client();

    // Create via v2 with tags and due_date
    let task: Value = c
        .post(format!("{base}/v2/tasks"))
        .json(&json!({
            "title": "Cross-version",
            "tags": ["test"],
            "due_date": "2025-06-15T00:00:00Z",
            "priority": "critical"
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let id = task["id"].as_str().unwrap();

    // Read via v1 — should see the task but without tags/due_date
    let v1_task: Value = c
        .get(format!("{base}/v1/tasks/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(v1_task["title"], "Cross-version");
    assert_eq!(v1_task["priority"], "critical");
    assert!(v1_task.get("tags").is_none(), "v1 should not expose tags");
}

#[tokio::test]
async fn test_v1_task_readable_via_v2_with_defaults() {
    let base = start_server().await;
    let c = client();

    // Create via v1 (no tags/due_date)
    let task: Value = c
        .post(format!("{base}/v1/tasks"))
        .json(&json!({"title": "V1 only"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let id = task["id"].as_str().unwrap();

    // Read via v2 — should see empty tags and null due_date
    let v2_task: Value = c
        .get(format!("{base}/v2/tasks/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(v2_task["title"], "V1 only");
    assert_eq!(v2_task["tags"], json!([]));
}

// =========================================================================
// Part 5: Deprecation headers
// =========================================================================

#[tokio::test]
async fn test_v1_responses_include_deprecation_headers() {
    let base = start_server().await;
    let resp = client()
        .get(format!("{base}/v1/tasks"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let headers = resp.headers();
    assert!(
        headers.get("sunset").is_some(),
        "v1 response should include Sunset header"
    );
    assert!(
        headers.get("deprecation").is_some(),
        "v1 response should include Deprecation header"
    );
}

#[tokio::test]
async fn test_v2_responses_do_not_include_deprecation_headers() {
    let base = start_server().await;
    let resp = client()
        .get(format!("{base}/v2/tasks"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let headers = resp.headers();
    assert!(
        headers.get("sunset").is_none(),
        "v2 response should NOT include Sunset header"
    );
    assert!(
        headers.get("deprecation").is_none(),
        "v2 response should NOT include Deprecation header"
    );
}
