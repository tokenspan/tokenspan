use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::api::models::UserRole;
use crate::api::provider::dto::{ProviderCreateInput, ProviderUpdateInput};
use crate::api::provider::provider_model::Provider;
use crate::api::services::ProviderServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ProviderMutation;

#[Object]
impl ProviderMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_provider<'a>(
        &self,
        ctx: &Context<'a>,
        input: ProviderCreateInput,
    ) -> Result<Provider> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let provider = provider_service.create(input).await?;

        Ok(provider)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_provider<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ProviderUpdateInput,
    ) -> Result<Provider> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let provider = provider_service.update_by_id(&id, input).await?;

        Ok(provider)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_provider<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Provider> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let provider = provider_service.delete_by_id(&id).await?;

        Ok(provider)
    }
}
