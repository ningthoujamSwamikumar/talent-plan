use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::graphql::types::GqlProject;
use crate::repository::ProjectRepository;

/// DataLoader for fetching projects by ID in batches, preventing N+1 queries.
pub struct ProjectLoader {
    pub repo: Arc<dyn ProjectRepository>,
}

impl ProjectLoader {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }
}

impl Loader<Uuid> for ProjectLoader {
    type Value = GqlProject;
    type Error = Arc<dyn std::error::Error + Send + Sync>;

    fn load(
        &self,
        keys: &[Uuid],
    ) -> impl std::future::Future<Output = Result<HashMap<Uuid, Self::Value>, Self::Error>> + Send
    {
        let repo = self.repo.clone();
        let keys = keys.to_vec();
        async move {
            let projects = repo
                .get_many(keys)
                .await
                .map_err(|e| Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>)?;

            Ok(projects
                .into_iter()
                .map(|(id, p)| (id, GqlProject::from(p)))
                .collect())
        }
    }
}
