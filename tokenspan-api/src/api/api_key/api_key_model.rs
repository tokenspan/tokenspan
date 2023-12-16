use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;

use crate::api::loaders::UserLoader;
use tokenspan_extra::pagination::{Cursor, CursorExt};

use crate::api::models::{Provider, ProviderId, User, UserId};
use crate::api::services::ProviderServiceDyn;
use crate::api::user::user_error::UserError;
use crate::error::AppError;

pub type ApiKeyId = Uuid;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ApiKey {
    pub id: ApiKeyId,
    pub name: String,
    #[graphql(skip)]
    pub key: String,
    pub owner_id: UserId,
    pub provider_id: ProviderId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl ApiKey {
    pub async fn provider<'a>(&self, ctx: &Context<'a>) -> Result<Option<Provider>> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let provider = provider_service
            .find_by_id(self.provider_id.clone())
            .await?;

        Ok(provider)
    }

    pub async fn owner<'a>(&self, ctx: &Context<'a>) -> Result<Option<User>> {
        let user_loader = ctx
            .data::<DataLoader<UserLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let user = user_loader
            .load_one(self.owner_id.clone())
            .await
            .map_err(|_| UserError::UserNotFound(Some(self.owner_id.clone())))?;

        Ok(user)
    }
}

impl CursorExt<Cursor> for ApiKey {
    fn cursor(&self) -> Cursor {
        self.created_at.clone().into()
    }
}

impl From<entity::api_key::Model> for ApiKey {
    fn from(value: entity::api_key::Model) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            key: value.key,
            owner_id: value.owner_id.into(),
            provider_id: value.provider_id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
