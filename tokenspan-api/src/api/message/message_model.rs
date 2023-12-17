use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub raw: Option<String>,
    pub content: String,
    pub role: String,
    pub task_version_id: Uuid,
}
impl From<entity::message::Model> for Message {
    fn from(value: entity::message::Model) -> Self {
        Self {
            id: value.id,
            raw: value.raw,
            content: value.content,
            role: value.role,
            task_version_id: value.task_version_id,
        }
    }
}
