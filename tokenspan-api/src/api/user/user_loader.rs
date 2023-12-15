use crate::api::models::{User, UserId};
use crate::loader::AppLoader;
use async_graphql::dataloader::Loader;
use std::collections::HashMap;

#[async_trait::async_trait]
impl Loader<UserId> for AppLoader {
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
