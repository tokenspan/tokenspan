use async_graphql::{Context, Object, Result};
use chrono::Utc;
use uuid::Uuid;

use crate::domains::models::UserRole;
use crate::domains::user::dto::UserCreateInput;
use crate::domains::user::user_model::User;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user<'a>(&self, _ctx: &Context<'a>, input: UserCreateInput) -> Result<User> {
        Ok(User {
            id: Uuid::new_v4(),
            email: input.email,
            username: input.username,
            password: input.password,
            salt: "".to_string(),
            role: UserRole::User,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
}
