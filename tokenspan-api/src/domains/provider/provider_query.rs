use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::domains::provider::dto::ProviderArgs;
use crate::domains::provider::provider_model::Provider;
use crate::domains::services::ProviderServiceDyn;
use crate::errors::AppError;
use dojo_orm::pagination::{AdditionalFields, Cursor};

#[derive(Default)]
pub struct ProviderQuery;

#[Object]
impl ProviderQuery {
    pub async fn providers<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: ProviderArgs,
    ) -> Result<Connection<Cursor, Provider, AdditionalFields>> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_provider = provider_service.paginate(args).await?;

        Ok(paginated_provider.into())
    }

    pub async fn provider<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<Provider>> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let provider = provider_service.find_by_id(&id).await?;

        Ok(provider)
    }
}
