use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use dojo_orm::pagination::{AdditionalFields, Cursor};
use uuid::Uuid;

use crate::api::parameter::dto::ParameterArgs;
use crate::api::parameter::parameter_model::Parameter;
use crate::api::services::ParameterServiceDyn;
use crate::error::AppError;

#[derive(Default)]
pub struct ParameterQuery;

#[Object]
impl ParameterQuery {
    pub async fn parameters<'a>(
        &self,
        ctx: &Context<'a>,
        args: ParameterArgs,
    ) -> Result<Connection<Cursor, Parameter, AdditionalFields>> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_parameter = parameter_service.paginate(args).await?;

        Ok(paginated_parameter.into())
    }

    pub async fn parameter<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<Parameter>> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameter = parameter_service.find_by_id(&id).await?;

        Ok(parameter)
    }
}
