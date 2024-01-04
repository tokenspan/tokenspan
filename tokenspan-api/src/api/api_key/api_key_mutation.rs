use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyCreateInput, ApiKeyUpdateInput};
use crate::api::models::{ParsedToken, UserRole};
use crate::api::services::ApiKeyServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ApiKeyMutation;

#[Object]
impl ApiKeyMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_api_key<'a>(
        &self,
        ctx: &Context<'a>,
        input: ApiKeyCreateInput,
    ) -> Result<ApiKey> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let api_key = api_key_service
            .create(input, parsed_token.user_id.clone())
            .await?;

        Ok(api_key)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_api_key<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ApiKeyUpdateInput,
    ) -> Result<Option<ApiKey>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let api_key = api_key_service.update_by_id(id, input).await?;

        Ok(api_key)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_api_key<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<ApiKey>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let api_key = api_key_service.delete_by_id(id).await?;

        Ok(api_key)
    }
}
