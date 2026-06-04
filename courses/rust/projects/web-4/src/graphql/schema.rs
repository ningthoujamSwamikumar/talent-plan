use std::sync::Arc;

use async_graphql::*;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::graphql::types::*;
use crate::models::{
    CreateTaskParams, PaginationParams, Priority, TaskChangeEvent, TaskFilter, TaskStatus,
    UpdateTaskParams,
};
use crate::repository::TaskRepository;

/// The GraphQL query root.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Fetch a single task by ID.
    async fn task(&self, ctx: &Context<'_>, id: String) -> Result<GqlTask> {
        let repo = ctx.data::<Arc<dyn TaskRepository>>()?;
        let uuid = id
            .parse()
            .map_err(|_| Error::new("invalid task ID"))?;
        let task = repo.get(uuid).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(GqlTask::from(task))
    }

    /// List tasks with optional filtering and pagination.
    async fn tasks(
        &self,
        ctx: &Context<'_>,
        project_id: Option<String>,
        page: Option<i32>,
        per_page: Option<i32>,
        status: Option<String>,
    ) -> Result<GqlTaskList> {
        let repo = ctx.data::<Arc<dyn TaskRepository>>()?;

        let project_uuid = match project_id {
            Some(pid) => Some(pid.parse().map_err(|_| Error::new("invalid project_id"))?),
            None => None,
        };

        let status_filter = match status {
            Some(s) => Some(
                s.parse::<TaskStatus>()
                    .map_err(|e| Error::new(e))?,
            ),
            None => None,
        };

        let filter = TaskFilter {
            project_id: project_uuid,
            status: status_filter,
        };

        let pagination = PaginationParams {
            page: page.unwrap_or(1),
            per_page: per_page.unwrap_or(20),
        };

        let result = repo
            .list(filter, pagination)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(GqlTaskList {
            items: result.items.into_iter().map(GqlTask::from).collect(),
            total: result.total,
            page: result.page,
            per_page: result.per_page,
        })
    }
}

/// The GraphQL mutation root.
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new task.
    async fn create_task(&self, ctx: &Context<'_>, input: CreateTaskInput) -> Result<GqlTask> {
        let repo = ctx.data::<Arc<dyn TaskRepository>>()?;
        let event_tx = ctx.data::<broadcast::Sender<TaskChangeEvent>>()?;

        let project_id = input
            .project_id
            .parse()
            .map_err(|_| Error::new("invalid project_id"))?;

        let priority: Priority = input
            .priority
            .parse()
            .map_err(|e: String| Error::new(e))?;

        if input.title.is_empty() {
            return Err(Error::new("title must not be empty"));
        }

        let params = CreateTaskParams {
            project_id,
            title: input.title,
            description: input.description,
            priority,
        };

        let task = repo.create(params).await.map_err(|e| Error::new(e.to_string()))?;

        let _ = event_tx.send(TaskChangeEvent {
            event_type: crate::models::TaskEventType::Created,
            task: task.clone(),
        });

        Ok(GqlTask::from(task))
    }

    /// Update an existing task.
    async fn update_task(&self, ctx: &Context<'_>, input: UpdateTaskInput) -> Result<GqlTask> {
        let repo = ctx.data::<Arc<dyn TaskRepository>>()?;
        let event_tx = ctx.data::<broadcast::Sender<TaskChangeEvent>>()?;

        let id = input
            .id
            .parse()
            .map_err(|_| Error::new("invalid task ID"))?;

        let status = match input.status {
            Some(s) => Some(s.parse::<TaskStatus>().map_err(|e| Error::new(e))?),
            None => None,
        };

        let priority = match input.priority {
            Some(p) => Some(p.parse::<Priority>().map_err(|e| Error::new(e))?),
            None => None,
        };

        let params = UpdateTaskParams {
            title: input.title,
            description: input.description,
            status,
            priority,
        };

        let task = repo.update(id, params).await.map_err(|e| Error::new(e.to_string()))?;

        let _ = event_tx.send(TaskChangeEvent {
            event_type: crate::models::TaskEventType::Updated,
            task: task.clone(),
        });

        Ok(GqlTask::from(task))
    }

    /// Delete a task by ID.
    async fn delete_task(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let repo = ctx.data::<Arc<dyn TaskRepository>>()?;
        let event_tx = ctx.data::<broadcast::Sender<TaskChangeEvent>>()?;

        let uuid = id
            .parse()
            .map_err(|_| Error::new("invalid task ID"))?;

        // Get the task before deleting for the event.
        let task = repo.get(uuid).await.map_err(|e| Error::new(e.to_string()))?;

        let success = repo.delete(uuid).await.map_err(|e| Error::new(e.to_string()))?;

        if success {
            let _ = event_tx.send(TaskChangeEvent {
                event_type: crate::models::TaskEventType::Deleted,
                task,
            });
        }

        Ok(success)
    }
}

/// The GraphQL subscription root.
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// Subscribe to task events, optionally filtered by project ID.
    async fn task_events(
        &self,
        ctx: &Context<'_>,
        project_id: Option<String>,
    ) -> Result<impl futures_core::Stream<Item = GqlTaskEvent>> {
        let event_tx = ctx.data::<broadcast::Sender<TaskChangeEvent>>()?;
        let rx = event_tx.subscribe();

        let project_uuid = match project_id {
            Some(pid) => Some(pid.parse::<uuid::Uuid>().map_err(|_| Error::new("invalid project_id"))?),
            None => None,
        };

        let stream = BroadcastStream::new(rx).filter_map(move |result| {
            match result {
                Ok(event) => {
                    if let Some(pid) = project_uuid {
                        if event.task.project_id != pid {
                            return None;
                        }
                    }
                    Some(GqlTaskEvent {
                        event_type: event.event_type.to_string(),
                        task: GqlTask::from(event.task),
                    })
                }
                Err(_) => None,
            }
        });

        Ok(stream)
    }
}

/// The full async-graphql schema type alias.
pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

/// Build the GraphQL schema with shared state injected into context.
pub fn build_schema(
    task_repo: Arc<dyn TaskRepository>,
    event_tx: broadcast::Sender<TaskChangeEvent>,
) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(task_repo)
        .data(event_tx)
        .finish()
}
