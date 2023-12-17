use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::ApiKeyArgs;
use crate::api::services::ApiKeyServiceDyn;
use crate::error::AppError;
use tokenspan_extra::pagination::{AdditionalFields, Cursor};

#[derive(Default)]
pub struct ApiKeyQuery;

#[Object]
impl ApiKeyQuery {
    pub async fn api_keys<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: ApiKeyArgs,
    ) -> Result<Connection<Cursor, ApiKey, AdditionalFields>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_api_key = api_key_service.paginate(args).await?;

        Ok(paginated_api_key.into())
    }

    pub async fn api_key<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<ApiKey>> {
        let api_key_service = ctx
            .data::<ApiKeyServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let api_key = api_key_service.find_by_id(id).await?;

        Ok(api_key)
    }
}
