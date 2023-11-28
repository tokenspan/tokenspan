use std::sync::Arc;

use async_graphql::Result;
use prisma_client_rust::Direction;

use crate::api::models::{UserId, ViewId};
use crate::api::view::dto::{CreateViewInput, UpdateViewInput, ViewArgs};
use crate::api::view::view_error::ViewError;
use crate::api::view::view_model::View;
use crate::prisma::{user, view, PrismaClient};
use tokenspan_utils::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait ViewServiceExt {
    async fn get_views(&self, args: ViewArgs) -> Result<Pagination<Cursor, View>>;
    async fn get_view_by_id(&self, id: ViewId) -> Result<Option<View>>;
    async fn get_views_by_ids(&self, ids: Vec<ViewId>) -> Result<Vec<View>>;
    async fn count_views(&self) -> Result<i64>;
    async fn create_view(&self, input: CreateViewInput, owner_id: UserId) -> Result<View>;
    async fn update_view(&self, id: ViewId, input: UpdateViewInput) -> Result<View>;
    async fn delete_view(&self, id: ViewId) -> Result<View>;
}

pub type ViewServiceDyn = Arc<dyn ViewServiceExt + Send + Sync>;

pub struct ViewService {
    prisma: Arc<PrismaClient>,
}

impl ViewService {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }
}

#[async_trait::async_trait]
impl ViewServiceExt for ViewService {
    async fn get_views(&self, args: ViewArgs) -> Result<Pagination<Cursor, View>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .view()
            .find_many(vec![])
            .take(take + 1)
            .order_by(view::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take((take + 2) * -1)
                .cursor(view::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(view::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| ViewError::UnableToGetViews)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_view_by_id(&self, id: ViewId) -> Result<Option<View>> {
        let view = self
            .prisma
            .view()
            .find_unique(view::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ViewError::UnableToGetView)?
            .map(|view| view.into());

        Ok(view)
    }

    async fn get_views_by_ids(&self, ids: Vec<ViewId>) -> Result<Vec<View>> {
        let ids = ids
            .into_iter()
            .map(|id| view::id::equals(id.into()))
            .collect();
        let views = self
            .prisma
            .view()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| ViewError::UnableToGetViews)?
            .into_iter()
            .map(|view| view.into())
            .collect();

        Ok(views)
    }

    async fn count_views(&self) -> Result<i64> {
        let count = self
            .prisma
            .view()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| ViewError::UnableToCountViews)?;

        Ok(count)
    }

    async fn create_view(&self, input: CreateViewInput, owner_id: UserId) -> Result<View> {
        let created_view = self
            .prisma
            .view()
            .create(
                user::id::equals(owner_id.into()),
                input.name,
                vec![view::config::set(input.config)],
            )
            .exec()
            .await
            .map_err(|_| ViewError::UnableToCreateView)?;

        Ok(created_view.into())
    }

    async fn update_view(&self, id: ViewId, input: UpdateViewInput) -> Result<View> {
        let updated_view = self
            .prisma
            .view()
            .update(view::id::equals(id.into()), input.into())
            .exec()
            .await
            .map_err(|_| ViewError::UnableToUpdateView)?;

        Ok(updated_view.into())
    }

    async fn delete_view(&self, id: ViewId) -> Result<View> {
        let deleted_view = self
            .prisma
            .view()
            .delete(view::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ViewError::UnableToDeleteView)?;

        Ok(deleted_view.into())
    }
}

impl From<ViewService> for ViewServiceDyn {
    fn from(value: ViewService) -> Self {
        Arc::new(value) as Self
    }
}
