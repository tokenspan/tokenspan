use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::api::models::User;
use crate::api::services::UserServiceDyn;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<User>> {
        let user_service = ctx.data_unchecked::<UserServiceDyn>();

        let user = user_service.find_by_id(id).await?;

        Ok(user)
    }
}
