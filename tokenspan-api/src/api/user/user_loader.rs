use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::api::models::User;
use crate::api::services::UserServiceDyn;

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
    type Error = ();

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let users = self
            .user_service
            .find_by_ids(keys)
            .await
            .unwrap()
            .into_iter()
            .map(|user| (user.id.clone(), user))
            .collect();

        Ok(users)
    }
}
