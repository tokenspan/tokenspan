use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::api::models::UserRole;
use crate::api::user::dto::CreateUserInput;
use crate::api::user::user_model::User;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user<'a>(&self, _ctx: &Context<'a>, input: CreateUserInput) -> Result<User> {
        Ok(User {
            id: Uuid::new_v4(),
            email: input.email,
            username: input.username,
            password: input.password,
            salt: "".to_string(),
            role: UserRole::User,
        })
    }
}
