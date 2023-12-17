use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;

use tokenspan_extra::pagination::{Cursor, CursorExt};

#[derive(SimpleObject, Debug, Clone)]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Provider {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::provider::Model> for Provider {
    fn from(value: entity::provider::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
