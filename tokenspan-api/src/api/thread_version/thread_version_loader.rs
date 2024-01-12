use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::api::models::ThreadVersion;
use crate::api::services::ThreadVersionServiceDyn;
use crate::api::thread_version::thread_version_error::ThreadVersionError;

pub struct ThreadVersionLoader {
    pub thread_version_service: ThreadVersionServiceDyn,
}

impl ThreadVersionLoader {
    pub fn new(thread_version_service: ThreadVersionServiceDyn) -> Self {
        Self {
            thread_version_service,
        }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ThreadVersionLoader {
    type Value = ThreadVersion;
    type Error = Arc<ThreadVersionError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let thread_versions = self
            .thread_version_service
            .find_by_ids(keys)
            .await
            .map_err(|e| Arc::new(ThreadVersionError::Unknown(e)))?
            .into_iter()
            .map(|thread_version| (thread_version.id.clone(), thread_version))
            .collect();

        Ok(thread_versions)
    }
}
