use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::domains::models::User;
use crate::domains::services::UserServiceDyn;

pub struct UserLoader {
    pub user_service: UserServiceDyn,
}

impl UserLoader {
    pub fn new(user_service: UserServiceDyn) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UserLoader {
    type Value = User;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let users = self
            .user_service
            .find_by_ids(keys)
            .await
            .map_err(|e| Arc::new(e))?
            .into_iter()
            .map(|user| (user.id.clone(), user))
            .collect();

        Ok(users)
    }
}
