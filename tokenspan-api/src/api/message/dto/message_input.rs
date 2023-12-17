use async_graphql::InputObject;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

#[derive(InputObject, Clone)]
pub struct MessageCreateInput {
    pub raw: Option<String>,
    pub content: String,
    pub role: String,
    pub task_version_id: Uuid,
}

#[derive(InputObject)]
pub struct MessageUpdateInput {
    pub raw: Option<String>,
    pub content: Option<String>,
    pub role: Option<String>,
}

impl MessageUpdateInput {
    pub fn copy(self, model: &mut entity::message::ActiveModel) {
        if let Some(content) = self.content {
            model.content = Set(content)
        }

        if let Some(role) = self.role {
            model.role = Set(role)
        }

        model.raw = Set(self.raw)
    }
}
