use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::errors::TaskError;
use crate::models::*;
use crate::repository::Repository;

/// An in-memory task store.
///
/// This implementation is complete but contains intentional subtle bugs
/// that students should discover through testing.
#[derive(Debug, Default)]
pub struct InMemoryTaskStore {
    tasks: HashMap<Uuid, Task>,
}

impl InMemoryTaskStore {
    /// Create a new empty store.
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    /// Validate a CreateTask request.
    fn validate_create(&self, request: &CreateTask) -> Result<(), TaskError> {
        if request.title.trim().is_empty() {
            return Err(TaskError::ValidationError(
                "Title must not be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate an UpdateTask request.
    fn validate_update(&self, request: &UpdateTask) -> Result<(), TaskError> {
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(TaskError::ValidationError(
                    "Title must not be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl Repository for InMemoryTaskStore {
    fn create(&mut self, request: CreateTask) -> Result<Task, TaskError> {
        self.validate_create(&request)?;

        let now = Utc::now();
        let task = Task {
            id: Uuid::new_v4(),
            title: request.title,
            description: request.description,
            status: TaskStatus::Todo,
            priority: request.priority,
            created_at: now,
            updated_at: now,
        };

        self.tasks.insert(task.id, task.clone());
        Ok(task)
    }

    fn get(&self, id: Uuid) -> Result<Option<Task>, TaskError> {
        Ok(self.tasks.get(&id).cloned())
    }

    fn list(&self, params: ListParams) -> Result<Vec<Task>, TaskError> {
        let mut tasks: Vec<Task> = self
            .tasks
            .values()
            .filter(|t| {
                if let Some(ref status) = params.status {
                    if &t.status != status {
                        return false;
                    }
                }
                if let Some(ref priority) = params.priority {
                    if &t.priority != priority {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by created_at descending for deterministic ordering.
        tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // BUG: Off-by-one in pagination. Uses `offset + 1` instead of `offset`,
        // which skips the first item when offset > 0.
        let start = if params.offset == 0 {
            0
        } else {
            params.offset + 1 // <-- intentional bug: off-by-one
        };

        let end = std::cmp::min(start + params.limit, tasks.len());

        if start >= tasks.len() {
            return Ok(Vec::new());
        }

        Ok(tasks[start..end].to_vec())
    }

    fn update(&mut self, id: Uuid, request: UpdateTask) -> Result<Task, TaskError> {
        self.validate_update(&request)?;

        let task = self
            .tasks
            .get_mut(&id)
            .ok_or(TaskError::NotFound(id))?;

        if let Some(title) = request.title {
            task.title = title;
        }
        if let Some(description) = request.description {
            task.description = description;
        }
        if let Some(priority) = request.priority {
            task.priority = priority;
        }
        task.updated_at = Utc::now();

        Ok(task.clone())
    }

    // BUG: Delete does not check whether the task exists. It returns Ok(())
    // even when no task was found. A correct implementation would return
    // Err(TaskError::NotFound(id)) if the ID is not in the store.
    fn delete(&mut self, id: Uuid) -> Result<(), TaskError> {
        self.tasks.remove(&id);
        Ok(())
    }

    fn transition_status(
        &mut self,
        id: Uuid,
        new_status: TaskStatus,
    ) -> Result<Task, TaskError> {
        let task = self
            .tasks
            .get_mut(&id)
            .ok_or(TaskError::NotFound(id))?;

        if !task.status.can_transition_to(&new_status) {
            return Err(TaskError::InvalidTransition {
                from: task.status,
                to: new_status,
            });
        }

        task.status = new_status;
        task.updated_at = Utc::now();

        Ok(task.clone())
    }

    fn count(&self) -> Result<usize, TaskError> {
        Ok(self.tasks.len())
    }
}
