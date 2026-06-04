//! In-memory data store using `RwLock<HashMap>`.

// These imports will be needed once you implement the store:
// use std::collections::HashMap;
// use std::sync::RwLock;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    CreateProject, CreateTask, PaginatedResponse, Project, Task, TaskFilterParams,
    UpdateProject, UpdateTask,
};

/// Application state holding the in-memory data store.
///
/// Wrap this in `Arc` before passing it to the router.
pub struct AppState {
    // TODO: students add fields
    //
    // Hint: you need two fields:
    //   projects: RwLock<HashMap<Uuid, Project>>,
    //   tasks: RwLock<HashMap<Uuid, Task>>,
    _private: (), // remove this once you add real fields
}

impl AppState {
    /// Creates a new empty store.
    pub fn new() -> Self {
        todo!("initialize the RwLock<HashMap> fields")
    }

    // -----------------------------------------------------------------------
    // Project methods
    // -----------------------------------------------------------------------

    /// Insert a new project into the store and return it.
    pub fn create_project(&self, _input: CreateProject) -> Result<Project, AppError> {
        todo!("create a Project from `input`, insert into the map, return it")
    }

    /// Retrieve a single project by ID.
    pub fn get_project(&self, _id: Uuid) -> Result<Project, AppError> {
        todo!("look up the project or return AppError::NotFound")
    }

    /// Return all projects as a `Vec`.
    pub fn list_projects(&self) -> Result<Vec<Project>, AppError> {
        todo!("collect all projects from the map")
    }

    /// Update an existing project. Returns the updated project.
    pub fn update_project(&self, _id: Uuid, _input: UpdateProject) -> Result<Project, AppError> {
        todo!("find the project, apply changes from `input`, update `updated_at`")
    }

    /// Delete a project by ID. Also deletes all tasks belonging to the project.
    pub fn delete_project(&self, _id: Uuid) -> Result<(), AppError> {
        todo!("remove the project and its tasks, or return NotFound")
    }

    // -----------------------------------------------------------------------
    // Task methods
    // -----------------------------------------------------------------------

    /// Create a new task under the given project.
    pub fn create_task(&self, _project_id: Uuid, _input: CreateTask) -> Result<Task, AppError> {
        todo!("verify project exists, create a Task, insert, return it")
    }

    /// Retrieve a single task by ID.
    pub fn get_task(&self, _id: Uuid) -> Result<Task, AppError> {
        todo!("look up the task or return AppError::NotFound")
    }

    /// List tasks for a project with optional filtering and pagination.
    pub fn list_tasks(
        &self,
        _project_id: Uuid,
        _filters: &TaskFilterParams,
    ) -> Result<PaginatedResponse<Task>, AppError> {
        todo!("filter tasks by project_id, status, priority; paginate the results")
    }

    /// Update an existing task. Returns the updated task.
    pub fn update_task(&self, _id: Uuid, _input: UpdateTask) -> Result<Task, AppError> {
        todo!("find the task, apply changes from `input`, update `updated_at`")
    }

    /// Delete a task by ID.
    pub fn delete_task(&self, _id: Uuid) -> Result<(), AppError> {
        todo!("remove the task or return NotFound")
    }
}
