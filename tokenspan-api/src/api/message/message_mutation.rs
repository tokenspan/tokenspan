use async_graphql::{Context, ErrorExtensions, Object};
use uuid::Uuid;

use crate::api::dto::{MessageCreateInput, MessageUpdateInput};
use crate::api::models::{Message, ParsedToken, UserRole};
use crate::api::services::MessageServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct MessageMutation;

#[Object]
impl MessageMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_message<'a>(
        &self,
        ctx: &Context<'a>,
        input: MessageCreateInput,
    ) -> async_graphql::Result<Message> {
        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let message_service = ctx
            .data::<MessageServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let message = message_service.create(input, parsed_token.user_id).await?;

        Ok(message)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_message<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: MessageUpdateInput,
    ) -> async_graphql::Result<Message> {
        let message_service = ctx
            .data::<MessageServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let message = message_service.update_by_id(&id, input).await?;

        Ok(message)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_message<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
    ) -> async_graphql::Result<Message> {
        let message_service = ctx
            .data::<MessageServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let message = message_service.delete_by_id(&id).await?;

        Ok(message)
    }
}
