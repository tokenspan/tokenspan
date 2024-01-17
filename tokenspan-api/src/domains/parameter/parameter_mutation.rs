use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::domains::models::UserRole;
use crate::domains::parameter::dto::{ParameterCreateInput, ParameterUpdateInput};
use crate::domains::parameter::parameter_model::Parameter;
use crate::domains::services::ParameterServiceDyn;
use crate::errors::AppError;
use crate::guards::RoleGuard;

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
    ) -> Result<Parameter> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameter = parameter_service.update_by_id(&id, input).await?;

        Ok(parameter)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_parameter<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Parameter> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameter = parameter_service.delete_by_id(&id).await?;

        Ok(parameter)
    }
}
