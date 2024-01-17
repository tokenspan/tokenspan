use crate::domains::dto::MessageArgs;
use crate::domains::models::Message;
use crate::domains::services::MessageServiceDyn;
use crate::errors::AppError;
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

        let message = message_service.find_by_id(&id).await?;

        Ok(message)
    }
}
