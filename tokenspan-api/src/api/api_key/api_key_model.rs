use std::fmt::Display;

use async_graphql::dataloader::DataLoader;
use async_graphql::{
    ComplexObject, Context, InputValueError, InputValueResult, Result, Scalar, ScalarType,
    SimpleObject, Value,
};
use chrono::{DateTime, FixedOffset};

use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::models::{Provider, ProviderId, User, UserId};
use crate::api::services::ProviderServiceDyn;
use crate::api::user::user_error::UserError;
use crate::error::AppError;
use crate::loader::AppLoader;
use crate::prisma::api_key;

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ApiKeyId(pub String);

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ApiKey {
    pub id: ApiKeyId,
    pub name: String,
    pub key: String,
    pub owner_id: UserId,
    pub provider_id: ProviderId,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[ComplexObject]
impl ApiKey {
    pub async fn provider<'a>(&self, ctx: &Context<'a>) -> Result<Option<Provider>> {
        let provider_service = ctx
            .data::<ProviderServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        provider_service
            .get_provider_by_id(self.provider_id.clone())
            .await
    }

    pub async fn owner<'a>(&self, ctx: &Context<'a>) -> Result<Option<User>> {
        let app_loader = ctx
            .data::<DataLoader<AppLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let user = app_loader
            .load_one(self.owner_id.clone())
            .await
            .map_err(|_| UserError::UserNotFound(Some(self.owner_id.clone())))?;

        Ok(user)
    }
}

impl CursorExt<Cursor> for ApiKey {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<api_key::Data> for ApiKey {
    fn from(value: api_key::Data) -> Self {
        Self {
            id: ApiKeyId(value.id),
            name: value.name,
            key: value.key,
            owner_id: UserId(value.owner_id),
            provider_id: ProviderId(value.provider_id),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
