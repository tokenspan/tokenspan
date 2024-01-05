use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::api::function::dto::{FunctionCreateInput, FunctionUpdateInput};
use crate::api::function::function_model::Function;
use crate::api::models::{ParsedToken, UserRole};
use crate::api::services::FunctionServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

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
    ) -> Result<Option<Function>> {
        let function_service = ctx
            .data::<FunctionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let function = function_service.update_by_id(id, input).await?;

        Ok(function)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_function<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
    ) -> Result<Option<Function>> {
        let function_service = ctx
            .data::<FunctionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let function = function_service.delete_by_id(id).await?;

        Ok(function)
    }
}
