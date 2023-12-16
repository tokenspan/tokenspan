use std::collections::HashMap;

use async_graphql::dataloader::Loader;

use crate::api::models::{User, UserId};
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
impl Loader<UserId> for UserLoader {
    type Value = User;
    type Error = ();

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, Self::Value>, Self::Error> {
        let users = self
            .user_service
            .find_by_ids(keys.to_vec())
            .await
            .unwrap()
            .into_iter()
            .map(|user| (user.id.clone(), user))
            .collect();

        Ok(users)
    }
}
