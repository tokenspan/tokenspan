use crate::api::dto::UserArgs;
use crate::api::models::User;
use crate::api::services::UserServiceDyn;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use dojo_orm::pagination::{AdditionalFields, Cursor};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn users<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: UserArgs,
    ) -> Result<Connection<Cursor, User, AdditionalFields>> {
        let user_service = ctx
            .data::<UserServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let users = user_service.paginate(args).await?;

        Ok(users.into())
    }

    async fn user<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<User>> {
        let user_service = ctx.data_unchecked::<UserServiceDyn>();

        let user = user_service.find_by_id(&id).await?;

        Ok(user)
    }
}
