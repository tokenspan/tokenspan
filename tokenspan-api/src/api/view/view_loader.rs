use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use crate::api::models::{View, ViewId};
use crate::api::view::view_error::ViewError;
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<ViewId> for AppLoader {
    type Value = View;
    type Error = Arc<ViewError>;

    async fn load(&self, keys: &[ViewId]) -> Result<HashMap<ViewId, Self::Value>, Self::Error> {
        let views = self
            .view_service
            .get_views_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ViewError::Unknown(anyhow::anyhow!(e.message))))?
            .into_iter()
            .map(|view| (view.id.clone(), view))
            .collect();

        Ok(views)
    }
}
