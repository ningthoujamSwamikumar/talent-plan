use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::broadcast;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};

use crate::grpc::proto::task_service_server::TaskService;
use crate::grpc::proto::*;
use crate::models::{
    CreateTaskParams, PaginationParams, Priority, TaskChangeEvent, TaskEventType, TaskFilter,
    TaskStatus, UpdateTaskParams,
};
use crate::repository::TaskRepository;

/// The gRPC service implementation. Delegates to the shared repository layer.
pub struct TaskServiceImpl {
    pub repo: Arc<dyn TaskRepository>,
    pub event_tx: broadcast::Sender<TaskChangeEvent>,
}

impl TaskServiceImpl {
    pub fn new(
        repo: Arc<dyn TaskRepository>,
        event_tx: broadcast::Sender<TaskChangeEvent>,
    ) -> Self {
        Self { repo, event_tx }
    }

    fn task_to_response(task: &crate::models::Task) -> TaskResponse {
        TaskResponse {
            id: task.id.to_string(),
            project_id: task.project_id.to_string(),
            title: task.title.clone(),
            description: task.description.clone(),
            status: task.status.to_string(),
            priority: task.priority.to_string(),
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.to_rfc3339(),
        }
    }
}

#[tonic::async_trait]
impl TaskService for TaskServiceImpl {
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let req = request.into_inner();

        let project_id = req
            .project_id
            .parse()
            .map_err(|_| Status::invalid_argument("invalid project_id"))?;

        let priority: Priority = req
            .priority
            .parse()
            .map_err(|e: String| Status::invalid_argument(e))?;

        if req.title.is_empty() {
            return Err(Status::invalid_argument("title must not be empty"));
        }

        let params = CreateTaskParams {
            project_id,
            title: req.title,
            description: req.description,
            priority,
        };

        let task = self
            .repo
            .create(params)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Broadcast the event (ignore send failures — no receivers is fine).
        let _ = self.event_tx.send(TaskChangeEvent {
            event_type: TaskEventType::Created,
            task: task.clone(),
        });

        Ok(Response::new(Self::task_to_response(&task)))
    }

    async fn get_task(
        &self,
        request: Request<GetTaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let req = request.into_inner();
        let id = req
            .id
            .parse()
            .map_err(|_| Status::invalid_argument("invalid id"))?;

        let task = self.repo.get(id).await.map_err(|e| match e {
            crate::repository::RepositoryError::NotFound => {
                Status::not_found("task not found")
            }
            other => Status::internal(other.to_string()),
        })?;

        Ok(Response::new(Self::task_to_response(&task)))
    }

    async fn list_tasks(
        &self,
        request: Request<ListTasksRequest>,
    ) -> Result<Response<ListTasksResponse>, Status> {
        let req = request.into_inner();

        let project_id = if req.project_id.is_empty() {
            None
        } else {
            Some(
                req.project_id
                    .parse()
                    .map_err(|_| Status::invalid_argument("invalid project_id"))?,
            )
        };

        let status_filter = if req.status_filter.is_empty() {
            None
        } else {
            Some(
                req.status_filter
                    .parse::<TaskStatus>()
                    .map_err(|e| Status::invalid_argument(e))?,
            )
        };

        let page = if req.page <= 0 { 1 } else { req.page };
        let per_page = if req.per_page <= 0 { 20 } else { req.per_page };

        let filter = TaskFilter {
            project_id,
            status: status_filter,
        };
        let pagination = PaginationParams { page, per_page };

        let result = self
            .repo
            .list(filter, pagination)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let tasks = result.items.iter().map(Self::task_to_response).collect();

        Ok(Response::new(ListTasksResponse {
            tasks,
            total: result.total,
        }))
    }

    async fn update_task(
        &self,
        request: Request<UpdateTaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let req = request.into_inner();
        let id = req
            .id
            .parse()
            .map_err(|_| Status::invalid_argument("invalid id"))?;

        let status = match req.status {
            Some(s) => Some(
                s.parse::<TaskStatus>()
                    .map_err(|e| Status::invalid_argument(e))?,
            ),
            None => None,
        };

        let priority = match req.priority {
            Some(p) => Some(
                p.parse::<Priority>()
                    .map_err(|e| Status::invalid_argument(e))?,
            ),
            None => None,
        };

        let params = UpdateTaskParams {
            title: req.title,
            description: req.description,
            status,
            priority,
        };

        let task = self.repo.update(id, params).await.map_err(|e| match e {
            crate::repository::RepositoryError::NotFound => {
                Status::not_found("task not found")
            }
            other => Status::internal(other.to_string()),
        })?;

        let _ = self.event_tx.send(TaskChangeEvent {
            event_type: TaskEventType::Updated,
            task: task.clone(),
        });

        Ok(Response::new(Self::task_to_response(&task)))
    }

    async fn delete_task(
        &self,
        request: Request<DeleteTaskRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let id = req
            .id
            .parse()
            .map_err(|_| Status::invalid_argument("invalid id"))?;

        // Get the task before deletion so we can include it in the event.
        let task = self.repo.get(id).await.map_err(|e| match e {
            crate::repository::RepositoryError::NotFound => {
                Status::not_found("task not found")
            }
            other => Status::internal(other.to_string()),
        })?;

        let success = self.repo.delete(id).await.map_err(|e| match e {
            crate::repository::RepositoryError::NotFound => {
                Status::not_found("task not found")
            }
            other => Status::internal(other.to_string()),
        })?;

        if success {
            let _ = self.event_tx.send(TaskChangeEvent {
                event_type: TaskEventType::Deleted,
                task,
            });
        }

        Ok(Response::new(DeleteResponse { success }))
    }

    type WatchTasksStream =
        Pin<Box<dyn Stream<Item = Result<TaskEvent, Status>> + Send + 'static>>;

    async fn watch_tasks(
        &self,
        request: Request<WatchTasksRequest>,
    ) -> Result<Response<Self::WatchTasksStream>, Status> {
        let req = request.into_inner();
        let project_id_filter = if req.project_id.is_empty() {
            None
        } else {
            Some(
                req.project_id
                    .parse::<uuid::Uuid>()
                    .map_err(|_| Status::invalid_argument("invalid project_id"))?,
            )
        };

        let mut rx = self.event_tx.subscribe();

        let stream = async_stream::try_stream! {
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        // Filter by project if requested.
                        if let Some(pid) = project_id_filter {
                            if event.task.project_id != pid {
                                continue;
                            }
                        }
                        yield TaskEvent {
                            event_type: event.event_type.to_string(),
                            task: Some(TaskServiceImpl::task_to_response(&event.task)),
                        };
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("watcher lagged by {} events", n);
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        };

        Ok(Response::new(Box::pin(stream)))
    }
}
