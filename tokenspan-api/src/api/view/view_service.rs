use std::sync::Arc;

use async_graphql::Result;

use crate::api::models::{UserId, ViewId};
use crate::api::repositories::{ViewCreateEntity, ViewUpdateEntity};
use crate::api::view::dto::{ViewArgs, ViewCreateInput, ViewUpdateInput};
use crate::api::view::view_error::ViewError;
use crate::api::view::view_model::View;

use crate::repository::RootRepository;
use tokenspan_extra::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait ViewServiceExt {
    async fn get_views(&self, args: ViewArgs) -> Result<Pagination<Cursor, View>>;
    async fn get_view_by_id(&self, id: ViewId) -> Result<Option<View>>;
    async fn get_views_by_ids(&self, ids: Vec<ViewId>) -> Result<Vec<View>>;
    async fn count_views(&self) -> Result<u64>;
    async fn create_view(&self, input: ViewCreateInput, owner_id: UserId) -> Result<View>;
    async fn update_view(&self, id: ViewId, input: ViewUpdateInput) -> Result<Option<View>>;
    async fn delete_view(&self, id: ViewId) -> Result<Option<View>>;
}

pub type ViewServiceDyn = Arc<dyn ViewServiceExt + Send + Sync>;

pub struct ViewService {
    repository: Arc<RootRepository>,
}

impl ViewService {
    pub fn new(repository: Arc<RootRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ViewServiceExt for ViewService {
    async fn get_views(&self, args: ViewArgs) -> Result<Pagination<Cursor, View>> {
        let paginated = self
            .repository
            .view
            .paginate::<View>(args.into())
            .await
            .map_err(|e| ViewError::Unknown(anyhow::anyhow!(e)))?;

        Ok(paginated)
    }

    async fn get_view_by_id(&self, id: ViewId) -> Result<Option<View>> {
        let view = self
            .repository
            .view
            .find_by_id(id)
            .await
            .map_err(|e| ViewError::Unknown(anyhow::anyhow!(e)))?
            .map(|view| view.into());

        Ok(view)
    }

    async fn get_views_by_ids(&self, ids: Vec<ViewId>) -> Result<Vec<View>> {
        let views = self
            .repository
            .view
            .find_many_by_ids(ids)
            .await
            .map_err(|e| ViewError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|view| view.into())
            .collect();

        Ok(views)
    }

    async fn count_views(&self) -> Result<u64> {
        let count = self
            .repository
            .view
            .count()
            .await
            .map_err(|e| ViewError::Unknown(anyhow::anyhow!(e)))?;

        Ok(count)
    }

    async fn create_view(&self, input: ViewCreateInput, owner_id: UserId) -> Result<View> {
        let created_view = self
            .repository
            .view
            .create(ViewCreateEntity {
                owner_id,
                name: input.name,
                config: input.config,
            })
            .await
            .map_err(|e| ViewError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_view.into())
    }

    async fn update_view(&self, id: ViewId, input: ViewUpdateInput) -> Result<Option<View>> {
        let updated_view = self
            .repository
            .view
            .update_by_id(
                id,
                ViewUpdateEntity {
                    name: input.name,
                    config: input.config,
                },
            )
            .await
            .map_err(|e| ViewError::Unknown(anyhow::anyhow!(e)))?
            .map(|view| view.into());

        Ok(updated_view)
    }

    async fn delete_view(&self, id: ViewId) -> Result<Option<View>> {
        let deleted_view = self
            .repository
            .view
            .delete_by_id(id)
            .await
            .map_err(|e| ViewError::Unknown(anyhow::anyhow!(e)))?
            .map(|view| view.into());

        Ok(deleted_view)
    }
}

impl From<ViewService> for ViewServiceDyn {
    fn from(value: ViewService) -> Self {
        Arc::new(value) as Self
    }
}
