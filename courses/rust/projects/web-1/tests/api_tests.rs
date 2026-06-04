//! Integration tests for the TaskForge API.
//!
//! Each test starts the server on a random port, sends HTTP requests via
//! `reqwest`, and asserts on the responses.

use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use std::sync::Arc;
use taskforge_api::{app, store::AppState};
use tokio::net::TcpListener;

// ---------------------------------------------------------------------------
// Test helper
// ---------------------------------------------------------------------------

/// Start the app on a random port and return `(base_url, client)`.
async fn spawn_app() -> (String, Client) {
    let state = Arc::new(AppState::new());
    let app = app(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let base = format!("http://{addr}");
    let client = Client::new();
    (base, client)
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

#[tokio::test]
async fn health_check_returns_200() {
    let (base, client) = spawn_app().await;
    let res = client.get(format!("{base}/health")).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ok");
}

// ---------------------------------------------------------------------------
// Project CRUD
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_project_returns_201() {
    let (base, client) = spawn_app().await;
    let res = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "Alpha", "description": "First project" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["name"], "Alpha");
    assert_eq!(body["description"], "First project");
    assert!(body["id"].is_string());
    assert!(body["created_at"].is_string());
}

#[tokio::test]
async fn create_project_without_description() {
    let (base, client) = spawn_app().await;
    let res = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "Beta" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["name"], "Beta");
    assert!(body["description"].is_null());
}

#[tokio::test]
async fn get_project_returns_created_project() {
    let (base, client) = spawn_app().await;

    let create_res: Value = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "Gamma" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let id = create_res["id"].as_str().unwrap();

    let res = client
        .get(format!("{base}/projects/{id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["name"], "Gamma");
    assert_eq!(body["id"], id);
}

#[tokio::test]
async fn list_projects_returns_all() {
    let (base, client) = spawn_app().await;

    client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "P1" }))
        .send()
        .await
        .unwrap();
    client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "P2" }))
        .send()
        .await
        .unwrap();

    let res = client.get(format!("{base}/projects")).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    let arr = body.as_array().unwrap();
    assert_eq!(arr.len(), 2);
}

#[tokio::test]
async fn update_project_modifies_fields() {
    let (base, client) = spawn_app().await;

    let create_res: Value = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "Old Name" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let id = create_res["id"].as_str().unwrap();

    let res = client
        .put(format!("{base}/projects/{id}"))
        .json(&json!({ "name": "New Name", "description": "updated" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["name"], "New Name");
    assert_eq!(body["description"], "updated");
}

#[tokio::test]
async fn delete_project_returns_204() {
    let (base, client) = spawn_app().await;

    let create_res: Value = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "Doomed" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let id = create_res["id"].as_str().unwrap();

    let res = client
        .delete(format!("{base}/projects/{id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn get_deleted_project_returns_404() {
    let (base, client) = spawn_app().await;

    let create_res: Value = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "Ghost" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let id = create_res["id"].as_str().unwrap();

    client
        .delete(format!("{base}/projects/{id}"))
        .send()
        .await
        .unwrap();

    let res = client
        .get(format!("{base}/projects/{id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["error"]["code"], "NOT_FOUND");
}

#[tokio::test]
async fn get_nonexistent_project_returns_404() {
    let (base, client) = spawn_app().await;

    let fake_id = uuid::Uuid::new_v4();
    let res = client
        .get(format!("{base}/projects/{fake_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["error"]["code"], "NOT_FOUND");
}

// ---------------------------------------------------------------------------
// Task CRUD
// ---------------------------------------------------------------------------

/// Helper: create a project and return its ID.
async fn create_project(base: &str, client: &Client, name: &str) -> String {
    let res: Value = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": name }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    res["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn create_task_returns_201() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Proj").await;

    let res = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "Do something" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["title"], "Do something");
    assert_eq!(body["project_id"], project_id);
    assert_eq!(body["status"], "todo");
    assert_eq!(body["priority"], "medium");
}

#[tokio::test]
async fn create_task_with_status_and_priority() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Proj").await;

    let res = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({
            "title": "Urgent",
            "status": "in_progress",
            "priority": "critical"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "in_progress");
    assert_eq!(body["priority"], "critical");
}

#[tokio::test]
async fn create_task_for_nonexistent_project_returns_404() {
    let (base, client) = spawn_app().await;
    let fake_id = uuid::Uuid::new_v4();

    let res = client
        .post(format!("{base}/projects/{fake_id}/tasks"))
        .json(&json!({ "title": "Orphan" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_task_returns_task() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Proj").await;

    let create_res: Value = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "My Task" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let task_id = create_res["id"].as_str().unwrap();

    let res = client
        .get(format!("{base}/tasks/{task_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["title"], "My Task");
}

#[tokio::test]
async fn list_tasks_returns_tasks_for_project() {
    let (base, client) = spawn_app().await;
    let p1 = create_project(&base, &client, "P1").await;
    let p2 = create_project(&base, &client, "P2").await;

    // Two tasks in P1, one in P2
    client
        .post(format!("{base}/projects/{p1}/tasks"))
        .json(&json!({ "title": "T1" }))
        .send()
        .await
        .unwrap();
    client
        .post(format!("{base}/projects/{p1}/tasks"))
        .json(&json!({ "title": "T2" }))
        .send()
        .await
        .unwrap();
    client
        .post(format!("{base}/projects/{p2}/tasks"))
        .json(&json!({ "title": "T3" }))
        .send()
        .await
        .unwrap();

    let res = client
        .get(format!("{base}/projects/{p1}/tasks"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["total"], 2);
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn update_task_changes_status() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Proj").await;

    let create_res: Value = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "WIP" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let task_id = create_res["id"].as_str().unwrap();

    let res = client
        .put(format!("{base}/tasks/{task_id}"))
        .json(&json!({ "status": "done" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["status"], "done");
}

#[tokio::test]
async fn delete_task_returns_204() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Proj").await;

    let create_res: Value = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "Bye" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let task_id = create_res["id"].as_str().unwrap();

    let res = client
        .delete(format!("{base}/tasks/{task_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn get_deleted_task_returns_404() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Proj").await;

    let create_res: Value = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "Ephemeral" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let task_id = create_res["id"].as_str().unwrap();

    client
        .delete(format!("{base}/tasks/{task_id}"))
        .send()
        .await
        .unwrap();

    let res = client
        .get(format!("{base}/tasks/{task_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_project_cascades_to_tasks() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Cascade").await;

    let create_res: Value = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "Child" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let task_id = create_res["id"].as_str().unwrap();

    // Delete the project
    client
        .delete(format!("{base}/projects/{project_id}"))
        .send()
        .await
        .unwrap();

    // Task should also be gone
    let res = client
        .get(format!("{base}/tasks/{task_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

#[tokio::test]
async fn list_tasks_with_pagination() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Paged").await;

    // Create 5 tasks
    for i in 1..=5 {
        client
            .post(format!("{base}/projects/{project_id}/tasks"))
            .json(&json!({ "title": format!("Task {i}") }))
            .send()
            .await
            .unwrap();
    }

    // Request page 1 with 2 per page
    let res = client
        .get(format!(
            "{base}/projects/{project_id}/tasks?page=1&per_page=2"
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["total"], 5);
    assert_eq!(body["page"], 1);
    assert_eq!(body["per_page"], 2);
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn list_tasks_page_beyond_range() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Empty Page").await;

    // Create 2 tasks
    for i in 1..=2 {
        client
            .post(format!("{base}/projects/{project_id}/tasks"))
            .json(&json!({ "title": format!("Task {i}") }))
            .send()
            .await
            .unwrap();
    }

    // Page 10 with default per_page should return empty data
    let res = client
        .get(format!("{base}/projects/{project_id}/tasks?page=10"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["total"], 2);
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

// ---------------------------------------------------------------------------
// Filtering
// ---------------------------------------------------------------------------

#[tokio::test]
async fn list_tasks_filter_by_status() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "FilterProj").await;

    client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "A", "status": "todo" }))
        .send()
        .await
        .unwrap();
    client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "B", "status": "done" }))
        .send()
        .await
        .unwrap();
    client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "C", "status": "done" }))
        .send()
        .await
        .unwrap();

    let res = client
        .get(format!(
            "{base}/projects/{project_id}/tasks?status=done"
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["total"], 2);
    for item in body["data"].as_array().unwrap() {
        assert_eq!(item["status"], "done");
    }
}

#[tokio::test]
async fn list_tasks_filter_by_priority() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "PrioProj").await;

    client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "Low", "priority": "low" }))
        .send()
        .await
        .unwrap();
    client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "High", "priority": "high" }))
        .send()
        .await
        .unwrap();

    let res = client
        .get(format!(
            "{base}/projects/{project_id}/tasks?priority=high"
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["total"], 1);
    assert_eq!(body["data"][0]["priority"], "high");
}

// ---------------------------------------------------------------------------
// Validation errors
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_project_empty_name_returns_422() {
    let (base, client) = spawn_app().await;

    let res = client
        .post(format!("{base}/projects"))
        .json(&json!({ "name": "" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["error"]["code"], "VALIDATION_ERROR");
}

#[tokio::test]
async fn create_task_empty_title_returns_422() {
    let (base, client) = spawn_app().await;
    let project_id = create_project(&base, &client, "Proj").await;

    let res = client
        .post(format!("{base}/projects/{project_id}/tasks"))
        .json(&json!({ "title": "" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["error"]["code"], "VALIDATION_ERROR");
}

#[tokio::test]
async fn create_project_missing_name_returns_error() {
    let (base, client) = spawn_app().await;

    // Send a body without the required "name" field.
    let res = client
        .post(format!("{base}/projects"))
        .json(&json!({ "description": "no name" }))
        .send()
        .await
        .unwrap();

    // axum returns 422 for deserialization failures by default
    assert!(
        res.status() == StatusCode::UNPROCESSABLE_ENTITY
            || res.status() == StatusCode::BAD_REQUEST
    );
}

// ---------------------------------------------------------------------------
// Not found (generic)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn nonexistent_route_returns_404() {
    let (base, client) = spawn_app().await;

    let res = client
        .get(format!("{base}/does-not-exist"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_nonexistent_task_returns_404_json() {
    let (base, client) = spawn_app().await;
    let fake_id = uuid::Uuid::new_v4();

    let res = client
        .get(format!("{base}/tasks/{fake_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let body: Value = res.json().await.unwrap();
    assert_eq!(body["error"]["code"], "NOT_FOUND");
}

// ---------------------------------------------------------------------------
// CORS
// ---------------------------------------------------------------------------

#[tokio::test]
async fn cors_headers_present() {
    let (base, client) = spawn_app().await;

    let res = client
        .get(format!("{base}/health"))
        .header("Origin", "http://example.com")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    // The CORS layer should include an access-control-allow-origin header.
    assert!(
        res.headers().contains_key("access-control-allow-origin"),
        "Expected CORS header access-control-allow-origin to be present"
    );
}

// ---------------------------------------------------------------------------
// Misc
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_nonexistent_project_returns_404() {
    let (base, client) = spawn_app().await;
    let fake_id = uuid::Uuid::new_v4();

    let res = client
        .put(format!("{base}/projects/{fake_id}"))
        .json(&json!({ "name": "Nope" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_nonexistent_task_returns_404() {
    let (base, client) = spawn_app().await;
    let fake_id = uuid::Uuid::new_v4();

    let res = client
        .put(format!("{base}/tasks/{fake_id}"))
        .json(&json!({ "status": "done" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_nonexistent_project_returns_404() {
    let (base, client) = spawn_app().await;
    let fake_id = uuid::Uuid::new_v4();

    let res = client
        .delete(format!("{base}/projects/{fake_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_nonexistent_task_returns_404() {
    let (base, client) = spawn_app().await;
    let fake_id = uuid::Uuid::new_v4();

    let res = client
        .delete(format!("{base}/tasks/{fake_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
