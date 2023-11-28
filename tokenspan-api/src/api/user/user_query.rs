use crate::api::models::UserId;
use crate::api::services::UserServiceDyn;
use async_graphql::{Context, Object, Result};

use crate::api::user::user_model::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user<'a>(&self, ctx: &Context<'a>, id: UserId) -> Result<Option<User>> {
        let user_service = ctx.data_unchecked::<UserServiceDyn>();

        user_service.get_user_by_id(id).await
    }
}
