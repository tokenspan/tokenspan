use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::api::models::UserRole;
use crate::api::parameter::dto::{ParameterCreateInput, ParameterUpdateInput};
use crate::api::parameter::parameter_model::Parameter;
use crate::api::services::ParameterServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ParameterMutation;

#[Object]
impl ParameterMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_parameter<'a>(
        &self,
        ctx: &Context<'a>,
        input: ParameterCreateInput,
    ) -> Result<Parameter> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameter = parameter_service.create(input).await?;

        Ok(parameter)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_parameter<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ParameterUpdateInput,
    ) -> Result<Option<Parameter>> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameter = parameter_service.update_by_id(id, input).await?;

        Ok(parameter)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_parameter<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
    ) -> Result<Option<Parameter>> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameter = parameter_service.delete_by_id(id).await?;

        Ok(parameter)
    }
}
