use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::domains::function::dto::FunctionArgs;
use crate::domains::function::function_model::Function;
use crate::domains::services::FunctionServiceDyn;
use crate::errors::AppError;
use dojo_orm::pagination::{AdditionalFields, Cursor};

#[derive(Default)]
pub struct FunctionQuery;

#[Object]
impl FunctionQuery {
    pub async fn functions<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: FunctionArgs,
    ) -> Result<Connection<Cursor, Function, AdditionalFields>> {
        let function_service = ctx
            .data::<FunctionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_function = function_service.paginate(args).await?;

        Ok(paginated_function.into())
    }

    pub async fn function<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<Function>> {
        let function_service = ctx
            .data::<FunctionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let function = function_service.find_by_id(&id).await?;

        Ok(function)
    }
}
