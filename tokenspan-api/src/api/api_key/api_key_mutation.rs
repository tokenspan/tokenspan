use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyCreateInput, ApiKeyUpdateInput};
use crate::api::models::{ApiKeyId, ParsedToken};
use crate::api::services::ApiKeyServiceDyn;
use crate::api::types::Role;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ApiKeyMutation;

#[Object]
impl ApiKeyMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
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

        api_key_service
            .create_api_key(input, parsed_token.user_id.clone())
            .await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_api_key<'a>(
        &self,
        ctx: &Context<'a>,
        id: ApiKeyId,
        input: ApiKeyUpdateInput,
    ) -> Result<Option<ApiKey>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        api_key_service.update_api_key(id, input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_api_key<'a>(
        &self,
        ctx: &Context<'a>,
        id: ApiKeyId,
    ) -> Result<Option<ApiKey>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        api_key_service.delete_api_key(id).await
    }
}
