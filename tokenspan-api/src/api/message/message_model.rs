use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use dojo_macros::{EmbeddedModel, Model};
use dojo_orm::pagination::{Cursor, CursorExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize, Model, EmbeddedModel)]
#[dojo(name = "messages")]
pub struct Message {
    pub id: Uuid,
    pub thread_version_id: Uuid,
    pub owner_id: Uuid,
    pub raw: String,
    pub content: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for Message {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}
