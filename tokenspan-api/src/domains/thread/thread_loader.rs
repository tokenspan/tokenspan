use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::api::models::Thread;
use crate::api::services::ThreadServiceDyn;
use crate::api::thread::thread_error::ThreadError;

pub struct ThreadLoader {
    pub thread_service: ThreadServiceDyn,
}

impl ThreadLoader {
    pub fn new(thread_service: ThreadServiceDyn) -> Self {
        Self { thread_service }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ThreadLoader {
    type Value = Thread;
    type Error = Arc<ThreadError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let threads = self
            .thread_service
            .find_by_ids(keys)
            .await
            .map_err(|e| Arc::new(ThreadError::Unknown(e)))?
            .into_iter()
            .map(|thread| (thread.id.clone(), thread))
            .collect();

        Ok(threads)
    }
}
