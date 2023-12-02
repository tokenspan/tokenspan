use std::fmt::Display;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

#[derive(ID, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProviderId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for Provider {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<super::provider_repository::ProviderEntity> for Provider {
    fn from(value: super::provider_repository::ProviderEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
