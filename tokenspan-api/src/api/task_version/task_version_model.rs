use async_graphql::Result;
use async_graphql::{ComplexObject, Enum, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use strum_macros::EnumString;

use crate::api::models::{Message, Parameter, Task};
use tokenspan_extra::pagination::{Cursor, CursorExt};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct TaskVersion {
    pub id: Uuid,
    pub semver: String,
    pub version: u32,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub status: TaskVersionStatus,
    pub task_id: Uuid,
    #[graphql(skip)]
    pub messages: serde_json::Value,
    #[graphql(skip)]
    pub parameters: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl TaskVersion {
    pub async fn task<'a>(&self, ctx: &async_graphql::Context<'a>) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<crate::api::services::TaskServiceDyn>()
            .map_err(|_| crate::error::AppError::ContextExtractionError)?;

        let task = task_service.find_by_id(self.task_id).await?;

        Ok(task)
    }

    pub async fn parameters(&self) -> Result<Vec<Parameter>> {
        let parameters: Vec<Parameter> = serde_json::from_value(self.parameters.clone())?;

        Ok(parameters)
    }

    pub async fn messages(&self) -> Result<Vec<Message>> {
        let messages: Vec<Message> = serde_json::from_value(self.messages.clone())?;

        Ok(messages)
    }
}

impl CursorExt<Cursor> for TaskVersion {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::task_version::Model> for TaskVersion {
    fn from(value: entity::task_version::Model) -> Self {
        Self {
            id: value.id,
            semver: value.semver,
            version: value.version as u32,
            release_note: value.release_note,
            description: value.description,
            document: value.document,
            status: TaskVersionStatus::from(value.status),
            task_id: value.task_id,
            messages: value.messages,
            parameters: value.parameters,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString)]
#[graphql(remote = "entity::sea_orm_active_enums::TaskVersionStatus")]
pub enum TaskVersionStatus {
    #[strum(serialize = "DRAFT")]
    Draft,
    #[strum(serialize = "RELEASED")]
    Released,
}
