use async_graphql::{InputObject, SimpleObject};

/// GraphQL representation of a Task.
#[derive(Debug, Clone, SimpleObject)]
pub struct GqlTask {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::Task> for GqlTask {
    fn from(t: crate::models::Task) -> Self {
        Self {
            id: t.id.to_string(),
            project_id: t.project_id.to_string(),
            title: t.title,
            description: t.description,
            status: t.status.to_string(),
            priority: t.priority.to_string(),
            created_at: t.created_at.to_rfc3339(),
            updated_at: t.updated_at.to_rfc3339(),
        }
    }
}

/// GraphQL representation of a Project.
#[derive(Debug, Clone, SimpleObject)]
pub struct GqlProject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::Project> for GqlProject {
    fn from(p: crate::models::Project) -> Self {
        Self {
            id: p.id.to_string(),
            name: p.name,
            description: p.description,
            created_at: p.created_at.to_rfc3339(),
            updated_at: p.updated_at.to_rfc3339(),
        }
    }
}

/// Input for creating a task via GraphQL.
#[derive(Debug, InputObject)]
pub struct CreateTaskInput {
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub priority: String,
}

/// Input for updating a task via GraphQL.
#[derive(Debug, InputObject)]
pub struct UpdateTaskInput {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

/// Paginated task list response for GraphQL.
#[derive(Debug, Clone, SimpleObject)]
pub struct GqlTaskList {
    pub items: Vec<GqlTask>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

/// GraphQL representation of a task event (for subscriptions).
#[derive(Debug, Clone, SimpleObject)]
pub struct GqlTaskEvent {
    pub event_type: String,
    pub task: GqlTask,
}
