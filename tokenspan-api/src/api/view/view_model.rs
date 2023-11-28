use std::fmt::Display;

use async_graphql::dataloader::DataLoader;
use async_graphql::{
    ComplexObject, Context, InputValueError, InputValueResult, Result, Scalar, ScalarType,
    SimpleObject, Value,
};
use chrono::{DateTime, FixedOffset};

use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::models::{User, UserId};
use crate::api::user::user_error::UserError;
use crate::error::AppError;
use crate::loader::AppLoader;
use crate::prisma::view;

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ViewId(pub String);

#[derive(SimpleObject, Debug, Clone)]
#[graphql(complex)]
pub struct View {
    pub id: ViewId,
    pub name: String,
    pub config: Option<serde_json::Value>,
    pub owner_id: UserId,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[ComplexObject]
impl View {
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

impl CursorExt<Cursor> for View {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<view::Data> for View {
    fn from(value: view::Data) -> Self {
        Self {
            id: ViewId(value.id),
            name: value.name,
            config: value.config,
            owner_id: UserId(value.owner_id),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
