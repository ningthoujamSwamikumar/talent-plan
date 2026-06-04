//! Integration tests for the database layer.
//!
//! **Requirements to run these tests:**
//!
//! 1. A running PostgreSQL instance.
//! 2. A `DATABASE_URL` environment variable pointing to a **test** database,
//!    e.g. `postgres://postgres:postgres@localhost:5432/taskforge_test`.
//! 3. The test database should be disposable -- the test harness will apply
//!    migrations and may leave data behind.
//!
//! Run with:
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@localhost/taskforge_test cargo test
//! ```
//!
//! If no PostgreSQL is available the tests are skipped automatically (the
//! helper function returns early).

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

use taskforge_db::models::*;
use taskforge_db::repository::{ProjectRepository, TaskRepository};

// ---------------------------------------------------------------------------
// Test helper
// ---------------------------------------------------------------------------

/// Try to connect to the test database.  If `DATABASE_URL` is unset or the
/// connection fails, return `None` so the calling test can skip gracefully.
async fn setup_pool() -> Option<PgPool> {
    let url = match std::env::var("DATABASE_URL") {
        Ok(u) => u,
        Err(_) => {
            eprintln!("DATABASE_URL not set -- skipping database tests");
            return None;
        }
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .ok()?;

    // Run migrations so the schema is up to date
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");

    Some(pool)
}

/// Create a project with a unique name for test isolation.
fn test_create_project() -> CreateProject {
    CreateProject {
        name: format!("test-project-{}", Uuid::new_v4()),
        description: Some("integration test project".into()),
    }
}

/// Create a task request tied to a given project.
fn test_create_task(project_id: Uuid) -> CreateTask {
    CreateTask {
        project_id,
        title: format!("test-task-{}", Uuid::new_v4()),
        description: Some("integration test task".into()),
        status: None,
        priority: None,
    }
}

// Macro to skip a test when the pool is not available.
macro_rules! require_pool {
    () => {
        match setup_pool().await {
            Some(p) => p,
            None => return,
        }
    };
}

// ---------------------------------------------------------------------------
// Project repository tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_project_persists() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);
    let req = test_create_project();
    let name = req.name.clone();

    let project = repo.create(req).await.expect("create failed");
    assert_eq!(project.name, name);
    assert!(project.description.is_some());
}

#[tokio::test]
async fn test_get_project_by_id() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);

    let created = repo.create(test_create_project()).await.unwrap();
    let fetched = repo.get_by_id(created.id).await.unwrap();

    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().id, created.id);
}

#[tokio::test]
async fn test_get_project_not_found() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);

    let result = repo.get_by_id(Uuid::new_v4()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_list_projects_pagination() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);

    // Create 3 projects
    for _ in 0..3 {
        repo.create(test_create_project()).await.unwrap();
    }

    let (page, total) = repo
        .list(PaginationParams {
            offset: Some(0),
            limit: Some(2),
        })
        .await
        .unwrap();

    assert!(page.len() <= 2);
    assert!(total >= 3);
}

#[tokio::test]
async fn test_update_project_partial() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);

    let created = repo.create(test_create_project()).await.unwrap();
    let updated = repo
        .update(
            created.id,
            UpdateProject {
                name: Some("renamed".into()),
                description: None, // should keep original
            },
        )
        .await
        .unwrap();

    let updated = updated.expect("project should exist");
    assert_eq!(updated.name, "renamed");
    assert_eq!(updated.description, created.description);
}

#[tokio::test]
async fn test_update_nonexistent_project() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);

    let result = repo
        .update(
            Uuid::new_v4(),
            UpdateProject {
                name: Some("nope".into()),
                description: None,
            },
        )
        .await
        .unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn test_delete_project() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);

    let created = repo.create(test_create_project()).await.unwrap();
    let deleted = repo.delete(created.id).await.unwrap();
    assert!(deleted);

    let gone = repo.get_by_id(created.id).await.unwrap();
    assert!(gone.is_none());
}

#[tokio::test]
async fn test_delete_nonexistent_project() {
    let pool = require_pool!();
    let repo = ProjectRepository::new(pool);

    let deleted = repo.delete(Uuid::new_v4()).await.unwrap();
    assert!(!deleted);
}

#[tokio::test]
async fn test_delete_project_cascades_to_tasks() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();
    task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();

    // Delete project -- tasks should cascade
    project_repo.delete(project.id).await.unwrap();

    let (tasks, count) = task_repo
        .list_by_project(
            project.id,
            TaskFilter {
                status: None,
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(tasks.len(), 0);
    assert_eq!(count, 0);
}

// ---------------------------------------------------------------------------
// Task repository tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_task_valid_project() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    let task = task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();

    assert_eq!(task.project_id, project.id);
    assert_eq!(task.status, "todo");
    assert_eq!(task.priority, "medium");
}

#[tokio::test]
async fn test_create_task_invalid_project_fk_violation() {
    let pool = require_pool!();
    let task_repo = TaskRepository::new(pool);

    let result = task_repo.create(test_create_task(Uuid::new_v4())).await;
    assert!(result.is_err(), "should fail with FK violation");
}

#[tokio::test]
async fn test_get_task_by_id() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    let created = task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();

    let fetched = task_repo.get_by_id(created.id).await.unwrap();
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().id, created.id);
}

#[tokio::test]
async fn test_list_tasks_by_project() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    for _ in 0..3 {
        task_repo
            .create(test_create_task(project.id))
            .await
            .unwrap();
    }

    let (tasks, total) = task_repo
        .list_by_project(
            project.id,
            TaskFilter {
                status: None,
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(tasks.len(), 3);
    assert_eq!(total, 3);
}

#[tokio::test]
async fn test_list_tasks_with_status_filter() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();

    // Create a task then update its status
    let task = task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();
    task_repo
        .update(
            task.id,
            UpdateTask {
                title: None,
                description: None,
                status: Some("done".into()),
                priority: None,
            },
        )
        .await
        .unwrap();

    // Create another task left at default "todo"
    task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();

    let (done_tasks, done_count) = task_repo
        .list_by_project(
            project.id,
            TaskFilter {
                status: Some("done".into()),
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(done_tasks.len(), 1);
    assert_eq!(done_count, 1);
}

#[tokio::test]
async fn test_update_task_partial() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    let task = task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();

    let updated = task_repo
        .update(
            task.id,
            UpdateTask {
                title: Some("new title".into()),
                description: None,
                status: None,
                priority: None,
            },
        )
        .await
        .unwrap()
        .expect("task should exist");

    assert_eq!(updated.title, "new title");
    assert_eq!(updated.status, task.status);
}

#[tokio::test]
async fn test_delete_task() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    let task = task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();

    let deleted = task_repo.delete(task.id).await.unwrap();
    assert!(deleted);

    let gone = task_repo.get_by_id(task.id).await.unwrap();
    assert!(gone.is_none());
}

#[tokio::test]
async fn test_search_tasks_by_keyword() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();

    let unique_word = format!("xyzzy{}", Uuid::new_v4().simple());
    task_repo
        .create(CreateTask {
            project_id: project.id,
            title: format!("Find the {unique_word} artifact"),
            description: Some("A very special task".into()),
            status: None,
            priority: None,
        })
        .await
        .unwrap();

    // Also create a task that should NOT match
    task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();

    let (results, count) = task_repo
        .search(
            &unique_word,
            TaskFilter {
                status: None,
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(count, 1);
    assert_eq!(results.len(), 1);
    assert!(results[0].title.contains(&unique_word));
}

#[tokio::test]
async fn test_move_tasks_to_project_transaction() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let src = project_repo.create(test_create_project()).await.unwrap();
    let dst = project_repo.create(test_create_project()).await.unwrap();

    for _ in 0..3 {
        task_repo.create(test_create_task(src.id)).await.unwrap();
    }

    let moved = task_repo
        .move_tasks_to_project(MoveTasksRequest {
            from_project_id: src.id,
            to_project_id: dst.id,
        })
        .await
        .unwrap();

    assert_eq!(moved, 3);

    // Source project should now have 0 tasks
    let (src_tasks, _) = task_repo
        .list_by_project(
            src.id,
            TaskFilter {
                status: None,
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(src_tasks.len(), 0);

    // Destination should have 3
    let (dst_tasks, _) = task_repo
        .list_by_project(
            dst.id,
            TaskFilter {
                status: None,
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(dst_tasks.len(), 3);
}

#[tokio::test]
async fn test_move_tasks_invalid_destination_rolls_back() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let src = project_repo.create(test_create_project()).await.unwrap();
    task_repo.create(test_create_task(src.id)).await.unwrap();

    let result = task_repo
        .move_tasks_to_project(MoveTasksRequest {
            from_project_id: src.id,
            to_project_id: Uuid::new_v4(), // does not exist
        })
        .await;

    assert!(result.is_err(), "should fail when destination does not exist");

    // Source tasks should still be there (rollback)
    let (tasks, _) = task_repo
        .list_by_project(
            src.id,
            TaskFilter {
                status: None,
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(tasks.len(), 1);
}

#[tokio::test]
async fn test_count_by_status() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();

    // Create tasks with different statuses
    task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap(); // todo

    let t2 = task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();
    task_repo
        .update(
            t2.id,
            UpdateTask {
                title: None,
                description: None,
                status: Some("in_progress".into()),
                priority: None,
            },
        )
        .await
        .unwrap();

    let t3 = task_repo
        .create(test_create_task(project.id))
        .await
        .unwrap();
    task_repo
        .update(
            t3.id,
            UpdateTask {
                title: None,
                description: None,
                status: Some("done".into()),
                priority: None,
            },
        )
        .await
        .unwrap();

    let counts = task_repo.count_by_status(project.id).await.unwrap();

    // Should have entries for todo, in_progress, and done
    assert!(!counts.is_empty());
    let total: i64 = counts.iter().map(|c| c.count).sum();
    assert_eq!(total, 3);
}

#[tokio::test]
async fn test_concurrent_task_creation() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());

    let project = project_repo.create(test_create_project()).await.unwrap();

    // Spawn 10 concurrent task creations
    let mut handles = Vec::new();
    for _ in 0..10 {
        let p = pool.clone();
        let pid = project.id;
        handles.push(tokio::spawn(async move {
            let repo = TaskRepository::new(p);
            repo.create(CreateTask {
                project_id: pid,
                title: format!("concurrent-{}", Uuid::new_v4()),
                description: None,
                status: None,
                priority: None,
            })
            .await
        }));
    }

    let mut success = 0;
    for h in handles {
        if h.await.unwrap().is_ok() {
            success += 1;
        }
    }

    assert_eq!(success, 10, "all concurrent inserts should succeed");

    let task_repo = TaskRepository::new(pool);
    let (tasks, count) = task_repo
        .list_by_project(
            project.id,
            TaskFilter {
                status: None,
                priority: None,
                search: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(tasks.len(), 10);
    assert_eq!(count, 10);
}

#[tokio::test]
async fn test_create_task_with_custom_status_and_priority() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    let task = task_repo
        .create(CreateTask {
            project_id: project.id,
            title: "urgent task".into(),
            description: None,
            status: Some("in_progress".into()),
            priority: Some("critical".into()),
        })
        .await
        .unwrap();

    assert_eq!(task.status, "in_progress");
    assert_eq!(task.priority, "critical");
}

#[tokio::test]
async fn test_create_task_invalid_status_rejected() {
    let pool = require_pool!();
    let project_repo = ProjectRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool);

    let project = project_repo.create(test_create_project()).await.unwrap();
    let result = task_repo
        .create(CreateTask {
            project_id: project.id,
            title: "bad status".into(),
            description: None,
            status: Some("invalid_status".into()),
            priority: None,
        })
        .await;

    assert!(result.is_err(), "invalid status should be rejected by CHECK constraint");
}
