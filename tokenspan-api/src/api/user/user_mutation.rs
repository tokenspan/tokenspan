use crate::api::user::dto::CreateUserInput;
use crate::api::user::user_model::User;
use crate::prisma::PrismaClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user<'a>(&self, ctx: &Context<'a>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let password = "123".to_string();
        let salt = "1".to_string();
        let created_user = db
            .user()
            .create(input.email, password, salt, input.username, vec![])
            .exec()
            .await?;

        Ok(created_user.into())
    }
}
