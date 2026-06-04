use crate::grpc::proto::task_service_client::TaskServiceClient;
use crate::grpc::proto::*;
use tonic::transport::Channel;

/// A convenience wrapper around the generated gRPC client.
pub struct TaskClient {
    inner: TaskServiceClient<Channel>,
}

impl TaskClient {
    /// Connect to the gRPC server at the given address.
    pub async fn connect(addr: String) -> Result<Self, tonic::transport::Error> {
        let inner = TaskServiceClient::connect(addr).await?;
        Ok(Self { inner })
    }

    /// Create a new task.
    pub async fn create_task(
        &mut self,
        project_id: &str,
        title: &str,
        description: &str,
        priority: &str,
    ) -> Result<TaskResponse, tonic::Status> {
        let request = tonic::Request::new(CreateTaskRequest {
            project_id: project_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            priority: priority.to_string(),
        });
        let response = self.inner.create_task(request).await?;
        Ok(response.into_inner())
    }

    /// Get a task by ID.
    pub async fn get_task(&mut self, id: &str) -> Result<TaskResponse, tonic::Status> {
        let request = tonic::Request::new(GetTaskRequest {
            id: id.to_string(),
        });
        let response = self.inner.get_task(request).await?;
        Ok(response.into_inner())
    }

    /// List tasks with optional filters.
    pub async fn list_tasks(
        &mut self,
        project_id: &str,
        page: i32,
        per_page: i32,
        status_filter: &str,
    ) -> Result<ListTasksResponse, tonic::Status> {
        let request = tonic::Request::new(ListTasksRequest {
            project_id: project_id.to_string(),
            page,
            per_page,
            status_filter: status_filter.to_string(),
        });
        let response = self.inner.list_tasks(request).await?;
        Ok(response.into_inner())
    }

    /// Update a task.
    pub async fn update_task(
        &mut self,
        id: &str,
        title: Option<String>,
        description: Option<String>,
        status: Option<String>,
        priority: Option<String>,
    ) -> Result<TaskResponse, tonic::Status> {
        let request = tonic::Request::new(UpdateTaskRequest {
            id: id.to_string(),
            title,
            description,
            status,
            priority,
        });
        let response = self.inner.update_task(request).await?;
        Ok(response.into_inner())
    }

    /// Delete a task by ID.
    pub async fn delete_task(&mut self, id: &str) -> Result<DeleteResponse, tonic::Status> {
        let request = tonic::Request::new(DeleteTaskRequest {
            id: id.to_string(),
        });
        let response = self.inner.delete_task(request).await?;
        Ok(response.into_inner())
    }

    /// Watch task events (server-streaming). Returns the raw streaming response.
    pub async fn watch_tasks(
        &mut self,
        project_id: &str,
    ) -> Result<tonic::Streaming<TaskEvent>, tonic::Status> {
        let request = tonic::Request::new(WatchTasksRequest {
            project_id: project_id.to_string(),
        });
        let response = self.inner.watch_tasks(request).await?;
        Ok(response.into_inner())
    }
}
