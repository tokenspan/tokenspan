use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use dojo_orm::pagination::{AdditionalFields, Cursor};
use uuid::Uuid;

use crate::domains::model::dto::ModelArgs;
use crate::domains::model::model_model::Model;
use crate::domains::services::ModelServiceDyn;
use crate::errors::AppError;

#[derive(Default)]
pub struct ModelQuery;

#[Object]
impl ModelQuery {
    pub async fn models<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: ModelArgs,
    ) -> Result<Connection<Cursor, Model, AdditionalFields>> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_model = model_service.paginate(args).await?;

        Ok(paginated_model.into())
    }

    pub async fn model<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<Model>> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.find_by_id(&id).await?;

        Ok(model)
    }
}
