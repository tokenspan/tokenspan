use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use rabbit_macros::Model;
use rabbit_orm::pagination::{Cursor, CursorExt};
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, Model)]
#[rabbit(name = "providers")]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Provider {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}
