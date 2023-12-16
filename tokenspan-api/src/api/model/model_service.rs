use std::sync::Arc;

use anyhow::Result;
use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::model::dto::{ModelArgs, ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_error::ModelError;
use crate::api::model::model_model::Model;
use crate::api::models::ModelId;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>>;
    async fn find_by_id(&self, id: ModelId) -> Result<Option<Model>>;
    async fn find_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Model>>;
    async fn count(&self) -> Result<u64>;
    async fn create(&self, input: ModelCreateInput) -> Result<Model>;
    async fn update_by_id(&self, id: ModelId, input: ModelUpdateInput) -> Result<Model>;
    async fn delete_by_id(&self, id: ModelId) -> Result<Model>;
}

pub type ModelServiceDyn = Arc<dyn ModelServiceExt + Send + Sync>;

impl FromRef<AppState> for ModelServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.model_service.clone()
    }
}

pub struct ModelService {
    db: DatabaseConnection,
}

impl ModelService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ModelServiceExt for ModelService {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>> {
        let take = args.take.unwrap_or(10) as u64;
        let mut cursor = entity::model::Entity::find()
            .cursor_by(entity::model::Column::Id)
            .order_by_desc(entity::model::Column::Id)
            .limit(Some(take));

        if let Some(after) = args.after.clone() {
            cursor.after(after.id);
        }

        if let Some(before) = args.before.clone() {
            cursor.before(before.id);
        }

        let count = entity::model::Entity::find().count(&self.db).await?;
        let items = cursor
            .all(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|model| model.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(
            items,
            args.before,
            args.after,
            take as i64,
            count,
        ))
    }

    async fn find_by_id(&self, id: ModelId) -> Result<Option<Model>> {
        let model = entity::model::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .map(|model| model.into());

        Ok(model)
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

    async fn find_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>> {
        let ids = ids.into_iter().map(|id| id.to_string()).collect::<Vec<_>>();
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
            id: Set(ModelId::new_v4()),
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

    async fn update_by_id(&self, id: ModelId, input: ModelUpdateInput) -> Result<Model> {
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

    async fn delete_by_id(&self, id: ModelId) -> Result<Model> {
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
