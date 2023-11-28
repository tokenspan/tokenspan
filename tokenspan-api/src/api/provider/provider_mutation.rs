use async_graphql::{Context, Object, Result};

use crate::api::models::{ProviderId, Role};
use crate::api::provider::dto::{CreateProviderInput, UpdateProviderInput};
use crate::api::provider::provider_model::Provider;
use crate::api::services::ProviderServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ProviderMutation;

#[Object]
impl ProviderMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn create_provider<'a>(
        &self,
        ctx: &Context<'a>,
        input: CreateProviderInput,
    ) -> Result<Provider> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        provider_service.create_provider(input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_provider<'a>(
        &self,
        ctx: &Context<'a>,
        id: ProviderId,
        input: UpdateProviderInput,
    ) -> Result<Provider> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        provider_service.update_provider(id, input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_provider<'a>(&self, ctx: &Context<'a>, id: ProviderId) -> Result<Provider> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        provider_service.delete_provider(id).await
    }
}
