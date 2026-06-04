# Building Block Web-4: gRPC and GraphQL

**Prerequisites**: [Project: Auth & Authorization](../projects/web-3/README.md).

Before starting [Project: gRPC and GraphQL](../projects/web-4/README.md), complete
the readings and exercises below.

## What to read

- [Protocol Buffers Language Guide](https://protobuf.dev/programming-guides/proto3/).
  The schema definition language for gRPC. Learn message types, field types,
  enums, and service definitions.

- [gRPC Core Concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
  Understand unary RPCs, server streaming, client streaming, and bidirectional
  streaming. Focus on unary and server-streaming for this project.

- [Tonic documentation](https://docs.rs/tonic/latest/tonic/).
  The Rust gRPC framework. Built on tokio and tower (same ecosystem as axum).

- [async-graphql Book](https://async-graphql.github.io/async-graphql/en/index.html).
  The most popular GraphQL library for Rust. Read the sections on: defining schemas,
  queries, mutations, subscriptions, and dataloaders.

- [GraphQL Best Practices](https://graphql.org/learn/best-practices/).
  Official guide on designing GraphQL APIs.

## Key concepts

### REST vs gRPC vs GraphQL

| Aspect | REST | gRPC | GraphQL |
|--------|------|------|---------|
| Protocol | HTTP/JSON | HTTP/2 + Protobuf | HTTP/JSON |
| Schema | OpenAPI (optional) | .proto (required) | GraphQL SDL (required) |
| Best for | Web clients | Service-to-service | Flexible client queries |
| Streaming | SSE/WebSocket | Native | Subscriptions |
| Type safety | Runtime | Compile-time | Runtime |

### Protobuf to Rust (tonic-build)

```protobuf
// task.proto
service TaskService {
    rpc GetTask(GetTaskRequest) returns (TaskResponse);
}
```

Generates a Rust trait:
```rust
#[tonic::async_trait]
impl TaskService for MyService {
    async fn get_task(
        &self,
        request: Request<GetTaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        todo!()
    }
}
```

### GraphQL with async-graphql

```rust
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn task(&self, ctx: &Context<'_>, id: ID) -> Result<Task> {
        let repo = ctx.data::<TaskRepository>()?;
        repo.get_by_id(id.parse()?).await
    }
}
```

### The DataLoader Pattern

Without dataloader, fetching 20 tasks with their projects = 1 query for tasks + 20 queries for projects (N+1 problem).

With dataloader, it's 1 + 1 = 2 queries (batch the project lookups).

```rust
pub struct ProjectLoader(PgPool);

impl Loader<Uuid> for ProjectLoader {
    type Value = Project;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Project>, Self::Error> {
        // Single query: SELECT * FROM projects WHERE id = ANY($1)
    }
}
```

## Exercises

**Exercise 1**: Define a .proto file for a simple greeting service and generate
Rust code with `tonic-build`. Implement the server and client.

**Exercise 2**: Add server-streaming to your greeting service: given a name,
stream back greetings in different languages with a delay between each.

**Exercise 3**: Create a minimal async-graphql schema with one query and one
mutation. Serve it with axum using `async-graphql-axum`.

**Exercise 4**: Implement a dataloader that batches database lookups. Log the
SQL queries to verify batching is happening.

## You're ready when...

- [ ] You can define protobuf services and generate Rust code
- [ ] You can implement a tonic gRPC server and client
- [ ] You understand gRPC streaming
- [ ] You can define GraphQL schemas with async-graphql
- [ ] You understand the N+1 problem and how dataloaders solve it

Next: [Project: gRPC and GraphQL](../projects/web-4/README.md)
