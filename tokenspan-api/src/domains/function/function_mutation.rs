use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::domains::function::dto::{FunctionCreateInput, FunctionUpdateInput};
use crate::domains::function::function_model::Function;
use crate::domains::models::{ParsedToken, UserRole};
use crate::domains::services::FunctionServiceDyn;
use crate::errors::AppError;
use crate::guards::RoleGuard;

#[derive(Default)]
pub struct FunctionMutation;

#[Object]
impl FunctionMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_function<'a>(
        &self,
        ctx: &Context<'a>,
        input: FunctionCreateInput,
    ) -> Result<Function> {
        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let function_service = ctx
            .data::<FunctionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let function = function_service.create(input, parsed_token.user_id).await?;

        Ok(function)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_function<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: FunctionUpdateInput,
    ) -> Result<Function> {
        let function_service = ctx
            .data::<FunctionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let function = function_service.update_by_id(&id, input).await?;

        Ok(function)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_function<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Function> {
        let function_service = ctx
            .data::<FunctionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let function = function_service.delete_by_id(&id).await?;

        Ok(function)
    }
}
