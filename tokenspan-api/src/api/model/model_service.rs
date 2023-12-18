use std::sync::Arc;

use anyhow::Result;
use axum::extract::FromRef;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::model::dto::{ModelArgs, ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_error::ModelError;
use crate::api::model::model_model::Model;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Model>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Model>>;
    async fn count(&self) -> Result<u64>;
    async fn create(&self, input: ModelCreateInput) -> Result<Model>;
    async fn update_by_id(&self, id: Uuid, input: ModelUpdateInput) -> Result<Model>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Model>;
}

pub type ModelServiceDyn = Arc<dyn ModelServiceExt + Send + Sync>;

impl FromRef<AppState> for ModelServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.model_service.clone()
    }
}

#[derive(TypedBuilder)]
pub struct ModelService {
    db: DatabaseConnection,
}

#[async_trait::async_trait]
impl ModelServiceExt for ModelService {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>> {
        let take = args.take.unwrap_or(10);
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };
        let mut select = entity::model::Entity::find()
            .limit(Some(limit))
            .order_by_desc(entity::model::Column::CreatedAt);

        if let Some(after) = args.after.clone() {
            let after: NaiveDateTime = after.try_into()?;
            select = select.filter(entity::model::Column::CreatedAt.lte(after));
        }

        if let Some(before) = args.before.clone() {
            let before: NaiveDateTime = before.try_into()?;
            select = select.filter(entity::model::Column::CreatedAt.gte(before));
        }

        let count_fut = entity::model::Entity::find().count(&self.db);
        let select_fut = select.all(&self.db);

        let (count, items) = tokio::join!(count_fut, select_fut);

        let count = count.map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?;
        let items = items
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|execution| execution.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>> {
        let model = entity::model::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .map(|model| model.into());

        Ok(model)
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Model>> {
        let models = entity::model::Entity::find()
            .filter(entity::model::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|model| model.into())
            .collect();

        Ok(models)
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Model>> {
        let model = entity::model::Entity::find()
            .filter(entity::model::Column::Slug.eq(slug))
            .one(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .map(|model| model.into());

        Ok(model)
    }

    async fn count(&self) -> Result<u64> {
        let count = entity::model::Entity::find().count(&self.db).await?;

        Ok(count)
    }

    async fn create(&self, input: ModelCreateInput) -> Result<Model> {
        let input_pricing = serde_json::to_value(input.input_pricing).map_err(|e| {
            ModelError::Unknown(anyhow::anyhow!("Failed to serialize input_pricing: {}", e))
        })?;
        let output_pricing = serde_json::to_value(input.output_pricing).map_err(|e| {
            ModelError::Unknown(anyhow::anyhow!("Failed to serialize output_pricing: {}", e))
        })?;

        let created_model = entity::model::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(input.name),
            description: Set(input.description),
            slug: Set(input.slug),
            context: Set(input.context as i32),
            input_pricing: Set(input_pricing),
            output_pricing: Set(output_pricing),
            training_at: Set(input.training_at),
            provider_id: Set(input.provider_id.into()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_model)
    }

    async fn update_by_id(&self, id: Uuid, input: ModelUpdateInput) -> Result<Model> {
        let mut updated_model = entity::model::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ModelError::Unknown(anyhow::anyhow!("Model not found")))?
            .into_active_model();

        updated_model.updated_at = Set(Utc::now().naive_utc());

        if let Some(name) = input.name {
            updated_model.name = Set(name);
        }

        if let Some(description) = input.description {
            updated_model.description = Set(description);
        }

        if let Some(slug) = input.slug {
            updated_model.slug = Set(slug);
        }

        if let Some(context) = input.context {
            updated_model.context = Set(context as i32);
        }

        if let Some(input_pricing) = input.input_pricing {
            let input_price = serde_json::to_value(input_pricing).map_err(|e| {
                ModelError::Unknown(anyhow::anyhow!("Failed to serialize input_pricing: {}", e))
            })?;
            updated_model.input_pricing = Set(input_price);
        }

        if let Some(output_pricing) = input.output_pricing {
            let output_pricing = serde_json::to_value(output_pricing).map_err(|e| {
                ModelError::Unknown(anyhow::anyhow!("Failed to serialize output_pricing: {}", e))
            })?;
            updated_model.output_pricing = Set(output_pricing);
        }

        if let Some(training_at) = input.training_at {
            updated_model.training_at = Set(training_at);
        }

        let updated_model = updated_model
            .update(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .into();

        Ok(updated_model)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Model> {
        let deleted_model = entity::model::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ModelError::Unknown(anyhow::anyhow!("Model not found")))?;

        deleted_model
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?;

        Ok(deleted_model.into())
    }
}

impl From<ModelService> for ModelServiceDyn {
    fn from(value: ModelService) -> Self {
        Arc::new(value) as Self
    }
}
