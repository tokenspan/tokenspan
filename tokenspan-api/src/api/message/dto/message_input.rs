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

impl TryFrom<MessageUpsertInput> for MessageCreateInput {
    type Error = anyhow::Error;

    fn try_from(input: MessageUpsertInput) -> Result<Self, Self::Error> {
        Ok(Self {
            raw: input.raw,
            content: input
                .content
                .ok_or(anyhow::anyhow!("content is required"))?,
            role: input.role.ok_or(anyhow::anyhow!("role is required"))?,
            task_version_id: input.task_version_id,
        })
    }
}

#[derive(InputObject)]
pub struct MessageUpdateInput {
    pub raw: Option<String>,
    pub content: Option<String>,
    pub role: Option<String>,
    #[graphql(skip)]
    pub task_version_id: Uuid,
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

impl From<MessageUpsertInput> for MessageUpdateInput {
    fn from(input: MessageUpsertInput) -> Self {
        Self {
            raw: input.raw,
            content: input.content,
            role: input.role,
            task_version_id: input.task_version_id,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct MessageUpsertInput {
    pub id: Option<Uuid>,
    pub raw: Option<String>,
    pub content: Option<String>,
    pub role: Option<String>,
    #[graphql(skip)]
    pub task_version_id: Uuid,
}
