//! Acceptance tests for TaskForge Capstone.
//!
//! These tests exercise the HTTP API and WebSocket interface from the outside.
//! They do NOT import anything from the crate — they are pure black-box tests.
//!
//! Prerequisites:
//!   - A running TaskForge server (set TEST_SERVER_ADDR or default http://127.0.0.1:3000)
//!   - A PostgreSQL database with migrations applied
//!
//! Run with:
//!   TEST_SERVER_ADDR=http://127.0.0.1:3000 cargo test --test acceptance_tests

use reqwest::StatusCode;
use serde_json::{json, Value};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

mod helpers {
    use reqwest::Client;

    pub struct TestApp {
        pub addr: String,
        pub client: Client,
    }

    impl TestApp {
        pub async fn spawn() -> Self {
            let addr = std::env::var("TEST_SERVER_ADDR")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".into());
            let client = Client::new();
            Self { addr, client }
        }

        pub fn url(&self, path: &str) -> String {
            format!("{}{}", self.addr, path)
        }
    }

    /// Register a unique user and return (access_token, user_id).
    pub async fn register_and_login(
        app: &TestApp,
        role: Option<&str>,
    ) -> (String, String) {
        let unique = uuid::Uuid::new_v4().to_string();
        let email = format!("user-{}@test.com", &unique[..8]);
        let password = "Str0ngP@ssword!";

        let mut body = serde_json::json!({
            "email": email,
            "password": password,
            "display_name": format!("Test User {}", &unique[..8]),
        });
        if let Some(r) = role {
            body["role"] = serde_json::json!(r);
        }

        let res = app
            .client
            .post(app.url("/api/auth/register"))
            .json(&body)
            .send()
            .await
            .expect("register request failed");
        assert!(
            res.status().is_success(),
            "register failed: {}",
            res.status()
        );

        let login_body = serde_json::json!({
            "email": email,
            "password": password,
        });
        let res = app
            .client
            .post(app.url("/api/auth/login"))
            .json(&login_body)
            .send()
            .await
            .expect("login request failed");
        assert!(res.status().is_success(), "login failed: {}", res.status());

        let data: serde_json::Value = res.json().await.unwrap();
        let token = data["access_token"]
            .as_str()
            .expect("missing access_token")
            .to_string();
        let user_id = data["user_id"]
            .as_str()
            .unwrap_or("")
            .to_string();
        (token, user_id)
    }

    /// Create a project and return its id.
    pub async fn create_project(app: &TestApp, token: &str) -> String {
        let unique = uuid::Uuid::new_v4().to_string();
        let body = serde_json::json!({
            "name": format!("Project {}", &unique[..8]),
            "description": "A test project",
        });
        let res = app
            .client
            .post(app.url("/api/projects"))
            .bearer_auth(token)
            .json(&body)
            .send()
            .await
            .expect("create project request failed");
        assert_eq!(res.status().as_u16(), 201, "create project failed");
        let data: serde_json::Value = res.json().await.unwrap();
        data["id"].as_str().expect("missing project id").to_string()
    }

    /// Create a task in a project and return its id.
    pub async fn create_task(app: &TestApp, token: &str, project_id: &str) -> String {
        let unique = uuid::Uuid::new_v4().to_string();
        let body = serde_json::json!({
            "title": format!("Task {}", &unique[..8]),
            "description": "A test task",
            "priority": "medium",
        });
        let res = app
            .client
            .post(app.url(&format!("/api/projects/{}/tasks", project_id)))
            .bearer_auth(token)
            .json(&body)
            .send()
            .await
            .expect("create task request failed");
        assert!(
            res.status().is_success(),
            "create task failed: {}",
            res.status()
        );
        let data: serde_json::Value = res.json().await.unwrap();
        data["id"].as_str().expect("missing task id").to_string()
    }
}

use helpers::*;

// ===========================================================================
// Auth Tests (8)
// ===========================================================================

mod auth {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn register_user_succeeds() {
        let app = TestApp::spawn().await;
        let unique = Uuid::new_v4().to_string();
        let body = json!({
            "email": format!("reg-{}@test.com", &unique[..8]),
            "password": "Str0ngP@ssword!",
            "display_name": "New User",
        });

        let res = app
            .client
            .post(app.url("/api/auth/register"))
            .json(&body)
            .send()
            .await
            .unwrap();

        assert!(
            res.status().is_success(),
            "Expected success, got {}",
            res.status()
        );
        let data: Value = res.json().await.unwrap();
        assert!(data.get("id").is_some(), "Response should contain user id");
        assert!(data.get("email").is_some(), "Response should contain email");
    }

    #[tokio::test]
    #[ignore]
    async fn register_duplicate_email_fails() {
        let app = TestApp::spawn().await;
        let unique = Uuid::new_v4().to_string();
        let body = json!({
            "email": format!("dup-{}@test.com", &unique[..8]),
            "password": "Str0ngP@ssword!",
            "display_name": "Dup User",
        });

        // First registration succeeds.
        let res = app
            .client
            .post(app.url("/api/auth/register"))
            .json(&body)
            .send()
            .await
            .unwrap();
        assert!(res.status().is_success());

        // Second registration with same email fails.
        let res = app
            .client
            .post(app.url("/api/auth/register"))
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_eq!(
            res.status(),
            StatusCode::CONFLICT,
            "Duplicate email should return 409"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn login_succeeds_returns_token() {
        let app = TestApp::spawn().await;
        let unique = Uuid::new_v4().to_string();
        let email = format!("login-{}@test.com", &unique[..8]);
        let password = "Str0ngP@ssword!";

        // Register first.
        app.client
            .post(app.url("/api/auth/register"))
            .json(&json!({
                "email": email,
                "password": password,
                "display_name": "Login User",
            }))
            .send()
            .await
            .unwrap();

        // Login.
        let res = app
            .client
            .post(app.url("/api/auth/login"))
            .json(&json!({ "email": email, "password": password }))
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        let data: Value = res.json().await.unwrap();
        assert!(
            data["access_token"].is_string(),
            "Response should contain access_token"
        );
        assert!(
            data["refresh_token"].is_string(),
            "Response should contain refresh_token"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn login_wrong_password_fails() {
        let app = TestApp::spawn().await;
        let unique = Uuid::new_v4().to_string();
        let email = format!("wrongpw-{}@test.com", &unique[..8]);

        app.client
            .post(app.url("/api/auth/register"))
            .json(&json!({
                "email": email,
                "password": "Str0ngP@ssword!",
                "display_name": "WrongPw User",
            }))
            .send()
            .await
            .unwrap();

        let res = app
            .client
            .post(app.url("/api/auth/login"))
            .json(&json!({ "email": email, "password": "WrongPassword!" }))
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::UNAUTHORIZED,
            "Wrong password should return 401"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn protected_route_without_token_returns_401() {
        let app = TestApp::spawn().await;

        let res = app
            .client
            .get(app.url("/api/projects"))
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::UNAUTHORIZED,
            "Missing token should return 401"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn protected_route_with_valid_token_succeeds() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;

        let res = app
            .client
            .get(app.url("/api/projects"))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert!(
            res.status().is_success(),
            "Valid token should allow access, got {}",
            res.status()
        );
    }

    #[tokio::test]
    #[ignore]
    async fn admin_can_access_admin_route() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, Some("admin")).await;

        // Admin-only route: listing all users or an admin endpoint.
        // We test against a route that requires admin privileges.
        let res = app
            .client
            .get(app.url("/api/admin/users"))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert!(
            res.status().is_success(),
            "Admin should access admin route, got {}",
            res.status()
        );
    }

    #[tokio::test]
    #[ignore]
    async fn viewer_cannot_modify_resources() {
        let app = TestApp::spawn().await;
        let (viewer_token, _) = register_and_login(&app, Some("viewer")).await;

        let body = json!({
            "name": "Viewer Project",
            "description": "Should fail",
        });

        let res = app
            .client
            .post(app.url("/api/projects"))
            .bearer_auth(&viewer_token)
            .json(&body)
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::FORBIDDEN,
            "Viewer should not be able to create projects"
        );
    }
}

// ===========================================================================
// Project Tests (8)
// ===========================================================================

mod projects {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn create_project_returns_201() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;

        let body = json!({
            "name": "My Project",
            "description": "A great project",
        });

        let res = app
            .client
            .post(app.url("/api/projects"))
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 201);
        let data: Value = res.json().await.unwrap();
        assert!(data["id"].is_string());
        assert_eq!(data["name"], "My Project");
    }

    #[tokio::test]
    #[ignore]
    async fn get_project_returns_data() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        let res = app
            .client
            .get(app.url(&format!("/api/projects/{}", project_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        let data: Value = res.json().await.unwrap();
        assert_eq!(data["id"].as_str().unwrap(), project_id);
        assert!(data["name"].is_string());
        assert!(data["created_at"].is_string());
    }

    #[tokio::test]
    #[ignore]
    async fn list_projects_with_pagination() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;

        // Create 3 projects.
        for _ in 0..3 {
            create_project(&app, &token).await;
        }

        // Request page 1 with per_page=2.
        let res = app
            .client
            .get(app.url("/api/projects?page=1&per_page=2"))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        let data: Value = res.json().await.unwrap();

        // The response should have a list of items and pagination metadata.
        let items = data["items"]
            .as_array()
            .or_else(|| data["data"].as_array())
            .expect("Response should contain items/data array");
        assert!(
            items.len() <= 2,
            "Page should contain at most 2 items, got {}",
            items.len()
        );

        // There should be some indication of total or next page.
        let has_pagination = data.get("total").is_some()
            || data.get("page").is_some()
            || data.get("total_pages").is_some();
        assert!(has_pagination, "Response should include pagination metadata");
    }

    #[tokio::test]
    #[ignore]
    async fn update_project_by_owner() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        let body = json!({
            "name": "Updated Name",
            "description": "Updated description",
        });

        let res = app
            .client
            .put(app.url(&format!("/api/projects/{}", project_id)))
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .unwrap();

        assert!(
            res.status().is_success(),
            "Owner should be able to update project, got {}",
            res.status()
        );

        let data: Value = res.json().await.unwrap();
        assert_eq!(data["name"], "Updated Name");
    }

    #[tokio::test]
    #[ignore]
    async fn delete_project_cascades_tasks() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;
        let task_id = create_task(&app, &token, &project_id).await;

        // Delete the project.
        let res = app
            .client
            .delete(app.url(&format!("/api/projects/{}", project_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();
        assert!(
            res.status().is_success() || res.status() == StatusCode::NO_CONTENT,
            "Delete project failed: {}",
            res.status()
        );

        // The task should no longer exist.
        let res = app
            .client
            .get(app.url(&format!("/api/tasks/{}", task_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();
        assert_eq!(
            res.status(),
            StatusCode::NOT_FOUND,
            "Task should be deleted when project is deleted"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn non_owner_cannot_delete_project() {
        let app = TestApp::spawn().await;
        let (owner_token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &owner_token).await;

        // Register a different (non-admin) user.
        let (other_token, _) = register_and_login(&app, None).await;

        let res = app
            .client
            .delete(app.url(&format!("/api/projects/{}", project_id)))
            .bearer_auth(&other_token)
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::FORBIDDEN,
            "Non-owner should not delete project"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn project_not_found_returns_404() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let fake_id = Uuid::new_v4();

        let res = app
            .client
            .get(app.url(&format!("/api/projects/{}", fake_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore]
    async fn create_project_validation_error() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;

        // Missing required "name" field.
        let body = json!({ "description": "No name" });

        let res = app
            .client
            .post(app.url("/api/projects"))
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "Missing name should return 422, got {}",
            res.status()
        );
    }
}

// ===========================================================================
// Task Tests (8)
// ===========================================================================

mod tasks {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn create_task_in_project() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        let body = json!({
            "title": "Implement feature X",
            "description": "Detailed description here",
            "priority": "high",
        });

        let res = app
            .client
            .post(app.url(&format!("/api/projects/{}/tasks", project_id)))
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .unwrap();

        assert!(
            res.status().is_success(),
            "Create task failed: {}",
            res.status()
        );
        let data: Value = res.json().await.unwrap();
        assert!(data["id"].is_string());
        assert_eq!(data["title"], "Implement feature X");
        assert_eq!(data["status"], "todo");
        assert_eq!(data["priority"], "high");
    }

    #[tokio::test]
    #[ignore]
    async fn get_task_returns_data() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;
        let task_id = create_task(&app, &token, &project_id).await;

        let res = app
            .client
            .get(app.url(&format!("/api/tasks/{}", task_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        let data: Value = res.json().await.unwrap();
        assert_eq!(data["id"].as_str().unwrap(), task_id);
        assert!(data["title"].is_string());
        assert!(data["project_id"].is_string());
    }

    #[tokio::test]
    #[ignore]
    async fn list_tasks_with_status_filter() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        // Create two tasks, update one to "done".
        let task1 = create_task(&app, &token, &project_id).await;
        create_task(&app, &token, &project_id).await;

        app.client
            .put(app.url(&format!("/api/tasks/{}", task1)))
            .bearer_auth(&token)
            .json(&json!({ "status": "done" }))
            .send()
            .await
            .unwrap();

        // Filter by status=todo — should not include the done task.
        let res = app
            .client
            .get(app.url(&format!(
                "/api/projects/{}/tasks?status=todo",
                project_id
            )))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        let data: Value = res.json().await.unwrap();
        let items = data["items"]
            .as_array()
            .or_else(|| data["data"].as_array())
            .or_else(|| data.as_array())
            .expect("Response should contain a task list");

        for item in items {
            assert_eq!(
                item["status"], "todo",
                "Filtered list should only contain todo tasks"
            );
        }
    }

    #[tokio::test]
    #[ignore]
    async fn update_task_status() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;
        let task_id = create_task(&app, &token, &project_id).await;

        let res = app
            .client
            .put(app.url(&format!("/api/tasks/{}", task_id)))
            .bearer_auth(&token)
            .json(&json!({ "status": "in_progress" }))
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        let data: Value = res.json().await.unwrap();
        assert_eq!(data["status"], "in_progress");
    }

    #[tokio::test]
    #[ignore]
    async fn delete_task() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;
        let task_id = create_task(&app, &token, &project_id).await;

        let res = app
            .client
            .delete(app.url(&format!("/api/tasks/{}", task_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();
        assert!(
            res.status().is_success() || res.status() == StatusCode::NO_CONTENT,
            "Delete task failed: {}",
            res.status()
        );

        // Confirm deletion.
        let res = app
            .client
            .get(app.url(&format!("/api/tasks/{}", task_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore]
    async fn assign_task_to_user() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let (_, assignee_id) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;
        let task_id = create_task(&app, &token, &project_id).await;

        let res = app
            .client
            .put(app.url(&format!("/api/tasks/{}", task_id)))
            .bearer_auth(&token)
            .json(&json!({ "assignee_id": assignee_id }))
            .send()
            .await
            .unwrap();

        assert!(
            res.status().is_success(),
            "Assign task failed: {}",
            res.status()
        );
        let data: Value = res.json().await.unwrap();
        assert_eq!(
            data["assignee_id"].as_str().unwrap(),
            assignee_id,
            "Task should be assigned to the specified user"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn task_not_found_returns_404() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let fake_id = Uuid::new_v4();

        let res = app
            .client
            .get(app.url(&format!("/api/tasks/{}", fake_id)))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore]
    async fn create_task_validation_error() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        // Missing required "title" field.
        let body = json!({ "description": "No title" });

        let res = app
            .client
            .post(app.url(&format!("/api/projects/{}/tasks", project_id)))
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "Missing title should return 422, got {}",
            res.status()
        );
    }
}

// ===========================================================================
// Real-Time / WebSocket Tests (6)
// ===========================================================================

mod realtime {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    /// Build a WebSocket URL from the HTTP server address.
    fn ws_url(app: &TestApp, token: &str, project_id: &str) -> String {
        let addr = app
            .addr
            .replace("http://", "ws://")
            .replace("https://", "wss://");
        format!("{}/ws?token={}&project_id={}", addr, token, project_id)
    }

    #[tokio::test]
    #[ignore]
    async fn websocket_connects() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        let url = ws_url(&app, &token, &project_id);
        let result = connect_async(&url).await;
        assert!(result.is_ok(), "WebSocket connection should succeed");

        let (mut ws, _) = result.unwrap();
        ws.close(None).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn websocket_receives_task_created_event() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        let url = ws_url(&app, &token, &project_id);
        let (mut ws, _) = connect_async(&url).await.expect("WS connect failed");

        // Create a task — should trigger an event.
        create_task(&app, &token, &project_id).await;

        // Wait for the event (with timeout).
        let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws.next())
            .await
            .expect("Timed out waiting for WS message")
            .expect("Stream ended")
            .expect("WS error");

        if let Message::Text(text) = msg {
            let event: Value = serde_json::from_str(&text).expect("Event should be valid JSON");
            let event_type = event["event"]
                .as_str()
                .or_else(|| event["type"].as_str())
                .expect("Event should have a type/event field");
            assert!(
                event_type.contains("created") || event_type.contains("create"),
                "Expected task_created event, got: {}",
                event_type
            );
        } else {
            panic!("Expected text message, got: {:?}", msg);
        }

        ws.close(None).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn websocket_receives_task_updated_event() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;
        let task_id = create_task(&app, &token, &project_id).await;

        let url = ws_url(&app, &token, &project_id);
        let (mut ws, _) = connect_async(&url).await.expect("WS connect failed");

        // Update the task.
        app.client
            .put(app.url(&format!("/api/tasks/{}", task_id)))
            .bearer_auth(&token)
            .json(&json!({ "status": "done" }))
            .send()
            .await
            .unwrap();

        let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws.next())
            .await
            .expect("Timed out waiting for WS message")
            .expect("Stream ended")
            .expect("WS error");

        if let Message::Text(text) = msg {
            let event: Value = serde_json::from_str(&text).expect("Event should be valid JSON");
            let event_type = event["event"]
                .as_str()
                .or_else(|| event["type"].as_str())
                .expect("Event should have type");
            assert!(
                event_type.contains("updated") || event_type.contains("update"),
                "Expected task_updated event, got: {}",
                event_type
            );
        } else {
            panic!("Expected text message, got: {:?}", msg);
        }

        ws.close(None).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn websocket_only_receives_events_for_subscribed_project() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_a = create_project(&app, &token).await;
        let project_b = create_project(&app, &token).await;

        // Subscribe to project A only.
        let url_a = ws_url(&app, &token, &project_a);
        let (mut ws_a, _) = connect_async(&url_a).await.expect("WS connect failed");

        // Create a task in project B — should NOT trigger event on ws_a.
        create_task(&app, &token, &project_b).await;

        let result =
            tokio::time::timeout(std::time::Duration::from_secs(2), ws_a.next()).await;

        // Should timeout (no event received), which is the expected behavior.
        assert!(
            result.is_err(),
            "Should not receive events for a different project"
        );

        ws_a.close(None).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn websocket_multiple_clients_in_same_project() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        let url = ws_url(&app, &token, &project_id);
        let (mut ws1, _) = connect_async(&url).await.expect("WS1 connect failed");
        let (mut ws2, _) = connect_async(&url).await.expect("WS2 connect failed");

        // Create a task — both clients should receive the event.
        create_task(&app, &token, &project_id).await;

        let msg1 = tokio::time::timeout(std::time::Duration::from_secs(5), ws1.next())
            .await
            .expect("WS1 timed out")
            .expect("WS1 stream ended")
            .expect("WS1 error");

        let msg2 = tokio::time::timeout(std::time::Duration::from_secs(5), ws2.next())
            .await
            .expect("WS2 timed out")
            .expect("WS2 stream ended")
            .expect("WS2 error");

        assert!(
            matches!(msg1, Message::Text(_)),
            "Client 1 should receive text event"
        );
        assert!(
            matches!(msg2, Message::Text(_)),
            "Client 2 should receive text event"
        );

        ws1.close(None).await.ok();
        ws2.close(None).await.ok();
    }

    #[tokio::test]
    #[ignore]
    async fn websocket_disconnect_and_reconnect() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;
        let project_id = create_project(&app, &token).await;

        let url = ws_url(&app, &token, &project_id);

        // Connect, then disconnect.
        let (mut ws, _) = connect_async(&url).await.expect("WS connect failed");
        ws.close(None).await.ok();

        // Small delay to let server process the close.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Reconnect — should work.
        let (mut ws2, _) = connect_async(&url)
            .await
            .expect("Reconnection should succeed");

        // Create a task — reconnected client should receive event.
        create_task(&app, &token, &project_id).await;

        let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws2.next())
            .await
            .expect("Timed out on reconnected WS")
            .expect("Stream ended")
            .expect("WS error");

        assert!(
            matches!(msg, Message::Text(_)),
            "Reconnected client should receive events"
        );

        ws2.close(None).await.ok();
    }
}

// ===========================================================================
// Observability Tests (4)
// ===========================================================================

mod observability {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn health_live_returns_200() {
        let app = TestApp::spawn().await;

        let res = app
            .client
            .get(app.url("/health/live"))
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::OK,
            "Liveness check should return 200"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn health_ready_returns_200() {
        let app = TestApp::spawn().await;

        let res = app
            .client
            .get(app.url("/health/ready"))
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::OK,
            "Readiness check should return 200"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn metrics_endpoint_returns_prometheus_format() {
        let app = TestApp::spawn().await;

        let res = app
            .client
            .get(app.url("/metrics"))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = res.text().await.unwrap();

        // Prometheus exposition format uses lines like:
        //   # HELP metric_name description
        //   # TYPE metric_name type
        //   metric_name{label="value"} 123
        assert!(
            body.contains("# HELP") || body.contains("# TYPE") || body.contains("http_requests"),
            "Metrics endpoint should return Prometheus exposition format, got: {}",
            &body[..body.len().min(200)]
        );
    }

    #[tokio::test]
    #[ignore]
    async fn request_has_trace_id_header() {
        let app = TestApp::spawn().await;

        let res = app
            .client
            .get(app.url("/health/live"))
            .send()
            .await
            .unwrap();

        let has_request_id = res.headers().contains_key("x-request-id")
            || res.headers().contains_key("x-trace-id");
        assert!(
            has_request_id,
            "Response should include X-Request-Id or X-Trace-Id header. Headers: {:?}",
            res.headers()
        );
    }
}

// ===========================================================================
// Deployment / Infrastructure Tests (4)
// ===========================================================================

mod deployment {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn config_loads_from_environment() {
        // This test verifies the server started successfully, which means it loaded
        // its configuration. We check the health endpoint as a proxy.
        let app = TestApp::spawn().await;

        let res = app
            .client
            .get(app.url("/health/live"))
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.status(),
            StatusCode::OK,
            "Server should start with environment config"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn cors_headers_present() {
        let app = TestApp::spawn().await;

        let res = app
            .client
            .request(reqwest::Method::OPTIONS, app.url("/api/projects"))
            .header("Origin", "http://example.com")
            .header("Access-Control-Request-Method", "GET")
            .send()
            .await
            .unwrap();

        let has_cors = res
            .headers()
            .contains_key("access-control-allow-origin")
            || res
                .headers()
                .contains_key("access-control-allow-methods");
        assert!(
            has_cors,
            "CORS headers should be present on OPTIONS response. Headers: {:?}",
            res.headers()
        );
    }

    #[tokio::test]
    #[ignore]
    async fn rate_limiting_rejects_excess_requests() {
        let app = TestApp::spawn().await;

        // Send a burst of requests to trigger rate limiting.
        // We use the health endpoint to avoid auth complexity.
        let mut got_429 = false;
        for _ in 0..200 {
            let res = app
                .client
                .get(app.url("/health/live"))
                .send()
                .await
                .unwrap();
            if res.status() == StatusCode::TOO_MANY_REQUESTS {
                got_429 = true;
                break;
            }
        }

        assert!(
            got_429,
            "Rate limiter should eventually return 429 under burst load"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn graceful_content_type_json_default() {
        let app = TestApp::spawn().await;
        let (token, _) = register_and_login(&app, None).await;

        let res = app
            .client
            .get(app.url("/api/projects"))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();

        let content_type = res
            .headers()
            .get("content-type")
            .expect("Response should have content-type header")
            .to_str()
            .unwrap()
            .to_string();

        assert!(
            content_type.contains("application/json"),
            "API responses should default to application/json, got: {}",
            content_type
        );
    }
}
