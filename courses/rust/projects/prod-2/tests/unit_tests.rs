use testing_mastery::*;
use uuid::Uuid;

// ---- Helper factories ----

/// Create a valid CreateTask request with default values.
fn make_create_request() -> CreateTask {
    CreateTask {
        title: "Test Task".to_string(),
        description: Some("A test task description".to_string()),
        priority: Priority::Medium,
    }
}

/// Create an InMemoryTaskStore pre-populated with `n` tasks.
/// Returns the store and a Vec of the created tasks.
fn make_store_with_tasks(n: usize) -> (InMemoryTaskStore, Vec<Task>) {
    let mut store = InMemoryTaskStore::new();
    let mut tasks = Vec::new();
    for i in 0..n {
        let req = CreateTask {
            title: format!("Task {}", i),
            description: Some(format!("Description for task {}", i)),
            priority: Priority::Medium,
        };
        let task = store.create(req).unwrap();
        tasks.push(task);
    }
    (store, tasks)
}

// ---- Unit Test Stubs ----

/// Test that creating a task with a valid title succeeds and returns
/// a task with the correct title, a valid UUID, and Todo status.
#[test]
fn test_create_task_success() {
    todo!("Create a task with a valid title and verify the returned task has the correct title, a non-nil UUID, and status == Todo")
}

/// Test that creating a task with an empty title returns a
/// ValidationError.
#[test]
fn test_create_task_empty_title_fails() {
    todo!("Try to create a task with title \"\" and verify it returns Err(TaskError::ValidationError(...))")
}

/// Test that creating a task with a whitespace-only title returns a
/// ValidationError.
#[test]
fn test_create_task_whitespace_title_fails() {
    todo!("Try to create a task with title \"   \" and verify it returns Err(TaskError::ValidationError(...))")
}

/// Test that getting a task by its ID returns the correct task.
#[test]
fn test_get_task_by_id() {
    todo!("Create a task, then get it by ID. Verify the returned task matches the created one.")
}

/// Test that getting a task with a non-existent ID returns None.
#[test]
fn test_get_nonexistent_task() {
    todo!("Call get with a random UUID on an empty store. Verify it returns Ok(None).")
}

/// Test that valid status transitions succeed:
///   Todo -> InProgress
///   InProgress -> Done
#[test]
fn test_valid_status_transitions() {
    todo!("Create a task (starts as Todo), transition to InProgress, then to Done. Verify each transition succeeds and the status is updated.")
}

/// Test that invalid status transitions are rejected.
/// Specifically, Done -> Todo should NOT be allowed.
/// (Hint: this test should reveal the intentional bug in the library.)
#[test]
fn test_invalid_status_transition_done_to_todo() {
    todo!("Create a task, transition it to InProgress then Done. Then try to transition from Done -> Todo. This SHOULD return an error, but the library has a bug that allows it.")
}

/// Test that Priority ordering is correct:
///   Low < Medium < High < Critical
#[test]
fn test_priority_ordering() {
    todo!("Verify that Low < Medium < High < Critical using comparison operators.")
}
