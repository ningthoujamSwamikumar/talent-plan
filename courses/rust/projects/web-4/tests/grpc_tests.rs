use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast;
use tonic::transport::Server;

use taskforge_multi::grpc::proto::task_service_client::TaskServiceClient;
use taskforge_multi::grpc::proto::task_service_server::TaskServiceServer;
use taskforge_multi::grpc::proto::*;
use taskforge_multi::grpc::TaskServiceImpl;
use taskforge_multi::models::TaskChangeEvent;
use taskforge_multi::repository::InMemoryTaskRepository;

/// Start a gRPC server on an ephemeral port and return the address.
async fn start_test_server() -> String {
    let repo = Arc::new(InMemoryTaskRepository::new());
    let (event_tx, _) = broadcast::channel::<TaskChangeEvent>(256);
    let service = TaskServiceImpl::new(repo, event_tx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);

    tokio::spawn(async move {
        Server::builder()
            .add_service(TaskServiceServer::new(service))
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });

    // Give the server a moment to start.
    tokio::time::sleep(Duration::from_millis(50)).await;

    format!("http://{}", addr)
}

fn test_project_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[tokio::test]
async fn test_create_task_via_grpc() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let response = client
        .create_task(CreateTaskRequest {
            project_id: test_project_id(),
            title: "Build gRPC service".into(),
            description: "Implement the task service".into(),
            priority: "high".into(),
        })
        .await
        .unwrap();

    let task = response.into_inner();
    assert_eq!(task.title, "Build gRPC service");
    assert_eq!(task.status, "todo");
    assert_eq!(task.priority, "high");
    assert!(!task.id.is_empty());
}

#[tokio::test]
async fn test_get_task_via_grpc() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let created = client
        .create_task(CreateTaskRequest {
            project_id: test_project_id(),
            title: "Fetch me".into(),
            description: "A task to retrieve".into(),
            priority: "medium".into(),
        })
        .await
        .unwrap()
        .into_inner();

    let fetched = client
        .get_task(GetTaskRequest {
            id: created.id.clone(),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.title, "Fetch me");
}

#[tokio::test]
async fn test_list_tasks_via_grpc() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();
    let pid = test_project_id();

    for i in 0..5 {
        client
            .create_task(CreateTaskRequest {
                project_id: pid.clone(),
                title: format!("Task {}", i),
                description: "".into(),
                priority: "low".into(),
            })
            .await
            .unwrap();
    }

    let list = client
        .list_tasks(ListTasksRequest {
            project_id: pid,
            page: 1,
            per_page: 10,
            status_filter: "".into(),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(list.tasks.len(), 5);
    assert_eq!(list.total, 5);
}

#[tokio::test]
async fn test_update_task_via_grpc() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let created = client
        .create_task(CreateTaskRequest {
            project_id: test_project_id(),
            title: "Original title".into(),
            description: "".into(),
            priority: "low".into(),
        })
        .await
        .unwrap()
        .into_inner();

    let updated = client
        .update_task(UpdateTaskRequest {
            id: created.id.clone(),
            title: Some("Updated title".into()),
            description: None,
            status: Some("in_progress".into()),
            priority: None,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(updated.title, "Updated title");
    assert_eq!(updated.status, "in_progress");
}

#[tokio::test]
async fn test_delete_task_via_grpc() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let created = client
        .create_task(CreateTaskRequest {
            project_id: test_project_id(),
            title: "Delete me".into(),
            description: "".into(),
            priority: "low".into(),
        })
        .await
        .unwrap()
        .into_inner();

    let delete_resp = client
        .delete_task(DeleteTaskRequest {
            id: created.id.clone(),
        })
        .await
        .unwrap()
        .into_inner();

    assert!(delete_resp.success);

    // Verify it's gone.
    let err = client
        .get_task(GetTaskRequest { id: created.id })
        .await
        .unwrap_err();

    assert_eq!(err.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn test_watch_tasks_receives_events() {
    let repo = Arc::new(InMemoryTaskRepository::new());
    let (event_tx, _) = broadcast::channel::<TaskChangeEvent>(256);
    let service = TaskServiceImpl::new(repo, event_tx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);

    tokio::spawn(async move {
        Server::builder()
            .add_service(TaskServiceServer::new(service))
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(50)).await;
    let connect_addr = format!("http://{}", addr);

    let mut watcher_client = TaskServiceClient::connect(connect_addr.clone()).await.unwrap();
    let mut creator_client = TaskServiceClient::connect(connect_addr).await.unwrap();

    let pid = test_project_id();

    // Start watching.
    let mut stream = watcher_client
        .watch_tasks(WatchTasksRequest {
            project_id: pid.clone(),
        })
        .await
        .unwrap()
        .into_inner();

    // Give the stream a moment to set up.
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Create a task — should trigger an event.
    let created = creator_client
        .create_task(CreateTaskRequest {
            project_id: pid.clone(),
            title: "Watched task".into(),
            description: "".into(),
            priority: "high".into(),
        })
        .await
        .unwrap()
        .into_inner();

    // Receive the event.
    let event = tokio::time::timeout(Duration::from_secs(2), stream.message())
        .await
        .expect("timed out waiting for event")
        .unwrap()
        .expect("stream ended unexpectedly");

    assert_eq!(event.event_type, "created");
    assert_eq!(event.task.unwrap().id, created.id);
}

#[tokio::test]
async fn test_get_nonexistent_task_returns_not_found() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let err = client
        .get_task(GetTaskRequest {
            id: uuid::Uuid::new_v4().to_string(),
        })
        .await
        .unwrap_err();

    assert_eq!(err.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn test_create_task_invalid_priority() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let err = client
        .create_task(CreateTaskRequest {
            project_id: test_project_id(),
            title: "Bad priority".into(),
            description: "".into(),
            priority: "ultra_mega".into(),
        })
        .await
        .unwrap_err();

    assert_eq!(err.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_create_task_empty_title() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let err = client
        .create_task(CreateTaskRequest {
            project_id: test_project_id(),
            title: "".into(),
            description: "".into(),
            priority: "low".into(),
        })
        .await
        .unwrap_err();

    assert_eq!(err.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_create_task_invalid_project_id() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();

    let err = client
        .create_task(CreateTaskRequest {
            project_id: "not-a-uuid".into(),
            title: "Valid title".into(),
            description: "".into(),
            priority: "low".into(),
        })
        .await
        .unwrap_err();

    assert_eq!(err.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_list_tasks_with_status_filter() {
    let addr = start_test_server().await;
    let mut client = TaskServiceClient::connect(addr).await.unwrap();
    let pid = test_project_id();

    let created = client
        .create_task(CreateTaskRequest {
            project_id: pid.clone(),
            title: "Filter test".into(),
            description: "".into(),
            priority: "low".into(),
        })
        .await
        .unwrap()
        .into_inner();

    // Update one to "done".
    client
        .update_task(UpdateTaskRequest {
            id: created.id,
            title: None,
            description: None,
            status: Some("done".into()),
            priority: None,
        })
        .await
        .unwrap();

    // Create another that stays "todo".
    client
        .create_task(CreateTaskRequest {
            project_id: pid.clone(),
            title: "Still todo".into(),
            description: "".into(),
            priority: "low".into(),
        })
        .await
        .unwrap();

    // Filter for "done" only.
    let list = client
        .list_tasks(ListTasksRequest {
            project_id: pid,
            page: 1,
            per_page: 10,
            status_filter: "done".into(),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(list.tasks.len(), 1);
    assert_eq!(list.tasks[0].status, "done");
}
