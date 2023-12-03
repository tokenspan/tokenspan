use crate::api::models::ApiKeyId;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::ApiKeyArgs;
use crate::api::services::ApiKeyServiceDyn;
use crate::error::AppError;
use tokenspan_utils::pagination::{AdditionalFields, Cursor};

#[derive(Default)]
pub struct ApiKeyQuery;

#[Object]
impl ApiKeyQuery {
    pub async fn api_keys<'a>(
        &self,
        ctx: &Context<'a>,
        args: ApiKeyArgs,
    ) -> Result<Connection<Cursor, ApiKey, AdditionalFields>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_api_key = api_key_service.get_api_keys(args).await?;

        Ok(paginated_api_key.into())
    }

    pub async fn api_key<'a>(&self, ctx: &Context<'a>, id: ApiKeyId) -> Result<Option<ApiKey>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let api_key = api_key_service.get_api_key_by_id(id).await?;

        Ok(api_key)
    }
}
