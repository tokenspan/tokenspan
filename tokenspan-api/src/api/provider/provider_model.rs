use std::fmt::Debug;

use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_extra::pagination::{Cursor, CursorExt};
use tokenspan_macros::ID;

#[derive(ID, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ProviderId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub slug: String,
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
            slug: value.slug,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
