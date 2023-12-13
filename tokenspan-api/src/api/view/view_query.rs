use crate::api::models::ViewId;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use crate::api::services::ViewServiceDyn;
use crate::api::view::dto::ViewArgs;
use crate::api::view::view_model::View;
use crate::error::AppError;
use tokenspan_extra::pagination::{AdditionalFields, Cursor};

#[derive(Default)]
pub struct ViewQuery;

#[Object]
impl ViewQuery {
    pub async fn views<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: ViewArgs,
    ) -> Result<Connection<Cursor, View, AdditionalFields>> {
        let view_service = ctx
            .data::<ViewServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_view = view_service.get_views(args).await?;

        Ok(paginated_view.into())
    }

    pub async fn view<'a>(&self, ctx: &Context<'a>, id: ViewId) -> Result<Option<View>> {
        let view_service = ctx
            .data::<ViewServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let view = view_service.get_view_by_id(id).await?;

        Ok(view)
    }
}
