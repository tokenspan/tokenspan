use async_graphql::{Context, Object, Result};

use crate::api::models::{ParameterId, Role};
use crate::api::parameter::dto::{CreateParameterInput, UpdateParameterInput};
use crate::api::parameter::parameter_model::Parameter;
use crate::api::services::ParameterServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ParameterMutation;

#[Object]
impl ParameterMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn create_parameter<'a>(
        &self,
        ctx: &Context<'a>,
        input: CreateParameterInput,
    ) -> Result<Parameter> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        parameter_service.create_parameter(input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_parameter<'a>(
        &self,
        ctx: &Context<'a>,
        id: ParameterId,
        input: UpdateParameterInput,
    ) -> Result<Parameter> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        parameter_service.update_parameter(id, input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_parameter<'a>(
        &self,
        ctx: &Context<'a>,
        id: ParameterId,
    ) -> Result<Parameter> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        parameter_service.delete_parameter(id).await
    }
}
