use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::NaiveDateTime;
use dojo_macros::Model;
use dojo_orm::pagination::{Cursor, CursorExt};
use uuid::Uuid;

use crate::api::loaders::{ProviderLoader, UserLoader};
use crate::api::models::{Provider, User};
use crate::error::AppError;

#[derive(SimpleObject, Clone, Model)]
#[graphql(complex)]
#[dojo(name = "api_keys")]
pub struct ApiKey {
    pub id: Uuid,
    pub name: String,
    #[graphql(skip)]
    pub key: String,
    pub owner_id: Uuid,
    pub provider_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl ApiKey {
    pub async fn provider<'a>(&self, ctx: &Context<'a>) -> Result<Option<Provider>> {
        let provider_loader = ctx
            .data::<DataLoader<ProviderLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let provider = provider_loader.load_one(self.provider_id.clone()).await?;

        Ok(provider)
    }

    pub async fn owner<'a>(&self, ctx: &Context<'a>) -> Result<Option<User>> {
        let user_loader = ctx
            .data::<DataLoader<UserLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let user = user_loader.load_one(self.owner_id).await?;

        Ok(user)
    }
}

impl CursorExt<Cursor> for ApiKey {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}
