use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use typed_builder::TypedBuilder;

use crate::api::models::User;
use crate::repository::RepositoryExt;

#[derive(TypedBuilder)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

#[async_trait]
impl RepositoryExt<User> for UserRepository {
    fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    fn table() -> &'static str {
        "users"
    }
}
