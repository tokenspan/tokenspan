use crate::api::dto::MessageArgs;
use crate::api::models::Message;
use crate::api::services::MessageServiceDyn;
use crate::error::AppError;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object};
use dojo_orm::pagination::{AdditionalFields, Cursor};
use uuid::Uuid;

#[derive(Default)]
pub struct MessageQuery;

#[Object]
impl MessageQuery {
    pub async fn messages<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: MessageArgs,
    ) -> async_graphql::Result<Connection<Cursor, Message, AdditionalFields>> {
        let message_service = ctx
            .data::<MessageServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_message = message_service.paginate(args).await?;

        Ok(paginated_message.into())
    }

    pub async fn message<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
    ) -> async_graphql::Result<Option<Message>> {
        let message_service = ctx
            .data::<MessageServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let message = message_service.find_by_id(id).await?;

        Ok(message)
    }
}
