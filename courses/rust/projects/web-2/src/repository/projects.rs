use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateProject, PaginationParams, Project, UpdateProject};

/// Encapsulates all database operations for the `projects` table.
///
/// Students: implement every method marked with `todo!()`.  Each method should
/// use `sqlx::query_as` (runtime-checked variant) to execute a SQL statement
/// against `self.pool`.
#[derive(Clone)]
pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: PgPool) -> Self {
        todo!()
    }

    /// INSERT a project and return the full row.
    pub async fn create(&self, req: CreateProject) -> Result<Project, sqlx::Error> {
        todo!()
    }

    /// SELECT a single project by primary key.  Returns `None` when the id
    /// does not exist rather than failing.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
        todo!()
    }

    /// SELECT projects with OFFSET / LIMIT pagination.
    ///
    /// Returns the page of rows **and** the total count so the caller can
    /// build pagination metadata.
    pub async fn list(
        &self,
        pagination: PaginationParams,
    ) -> Result<(Vec<Project>, i64), sqlx::Error> {
        todo!()
    }

    /// UPDATE a project.  Only the fields that are `Some` in `req` should be
    /// changed; the others must keep their current values.
    ///
    /// Returns `None` if the project does not exist.
    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateProject,
    ) -> Result<Option<Project>, sqlx::Error> {
        todo!()
    }

    /// DELETE a project by id.  Returns `true` if a row was actually deleted.
    ///
    /// Because the schema uses `ON DELETE CASCADE`, all tasks belonging to the
    /// project are removed automatically.
    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        todo!()
    }
}
