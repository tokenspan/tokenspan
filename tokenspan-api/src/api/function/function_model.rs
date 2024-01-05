use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::Model;
use dojo_orm::pagination::{Cursor, CursorExt};
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, Model)]
#[dojo(name = "functions")]
pub struct Function {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub response: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Function {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}
