use std::fmt::Display;

use async_graphql::dataloader::DataLoader;
use async_graphql::{
    ComplexObject, Context, InputValueError, InputValueResult, Result, Scalar, ScalarType,
    SimpleObject, Value,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::models::{User, UserId};
use crate::api::user::user_error::UserError;
use crate::error::AppError;
use crate::loader::AppLoader;


#[derive(ID, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ViewId(ObjectId);

#[derive(SimpleObject, Debug, Clone)]
#[graphql(complex)]
pub struct View {
    pub id: ViewId,
    pub name: String,
    pub config: Option<serde_json::Value>,
    pub owner_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

impl From<super::view_repository::ViewDoc> for View {
    fn from(value: crate::api::repositories::ViewDoc) -> Self {
        Self {
            id: ViewId::try_from(value.id.to_string()).unwrap(),
            name: value.name,
            config: value.config,
            owner_id: UserId::try_from(value.owner_id.to_string()).unwrap(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
