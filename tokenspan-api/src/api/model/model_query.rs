use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use tokenspan_extra::pagination::{AdditionalFields, Cursor};

use crate::api::model::dto::ModelArgs;
use crate::api::model::model_model::Model;
use crate::api::models::ModelId;
use crate::api::services::ModelServiceDyn;
use crate::error::AppError;

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

    pub async fn model<'a>(&self, ctx: &Context<'a>, id: ModelId) -> Result<Option<Model>> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.find_by_id(id).await?;

        Ok(model)
    }
}
