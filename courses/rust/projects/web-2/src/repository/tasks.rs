use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    CreateTask, MoveTasksRequest, StatusCount, Task, TaskFilter, TaskWithProject, UpdateTask,
};

/// Encapsulates all database operations for the `tasks` table.
///
/// Students: implement every method marked with `todo!()`.  Pay special
/// attention to `move_tasks_to_project` which **must** use a database
/// transaction so that the operation is atomic.
#[derive(Clone)]
pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: PgPool) -> Self {
        todo!()
    }

    /// INSERT a task and return the full row.
    ///
    /// The `project_id` in `req` must reference an existing project; if it
    /// does not, PostgreSQL will raise a foreign-key violation which should
    /// propagate as `sqlx::Error`.
    pub async fn create(&self, req: CreateTask) -> Result<Task, sqlx::Error> {
        todo!()
    }

    /// SELECT a single task by primary key.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
        todo!()
    }

    /// List tasks belonging to a specific project, respecting optional status
    /// and priority filters plus pagination.
    pub async fn list_by_project(
        &self,
        project_id: Uuid,
        filter: TaskFilter,
    ) -> Result<(Vec<Task>, i64), sqlx::Error> {
        todo!()
    }

    /// Full-text search across task titles and descriptions using the
    /// `search_vector` column.  Returns matching tasks joined with the
    /// parent project name.
    pub async fn search(
        &self,
        query: &str,
        filter: TaskFilter,
    ) -> Result<(Vec<TaskWithProject>, i64), sqlx::Error> {
        todo!()
    }

    /// UPDATE a task.  Only `Some` fields are changed.
    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateTask,
    ) -> Result<Option<Task>, sqlx::Error> {
        todo!()
    }

    /// DELETE a task by id.  Returns `true` if a row was deleted.
    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        todo!()
    }

    /// **Transaction exercise** -- move every task from one project to another
    /// in a single atomic operation.
    ///
    /// Steps inside the transaction:
    /// 1. Verify both projects exist (SELECT ... FOR UPDATE).
    /// 2. UPDATE tasks SET project_id = to WHERE project_id = from.
    /// 3. Return the number of moved rows.
    ///
    /// If either project does not exist the transaction must be rolled back
    /// and an appropriate error returned.
    pub async fn move_tasks_to_project(
        &self,
        req: MoveTasksRequest,
    ) -> Result<u64, sqlx::Error> {
        todo!()
    }

    /// Aggregate: count tasks grouped by status for a given project.
    ///
    /// Example result: `[("todo", 3), ("in_progress", 2), ("done", 5)]`
    pub async fn count_by_status(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<StatusCount>, sqlx::Error> {
        todo!()
    }
}
