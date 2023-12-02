use crate::api::models::{Role, UserId};
use crate::api::user::dto::CreateUserInput;
use crate::api::user::user_model::User;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user<'a>(&self, _ctx: &Context<'a>, input: CreateUserInput) -> Result<User> {
        Ok(User {
            id: UserId::new(),
            email: input.email,
            password: input.password,
            salt: "".to_string(),
            role: Role::User,
        })
    }
}
