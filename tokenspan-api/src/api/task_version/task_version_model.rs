use anyhow::Result;
use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use strum_macros::EnumString;

use tokenspan_extra::pagination::{Cursor, CursorExt};

use crate::api::models::{Message, Parameter};
use crate::api::services::{MessageServiceDyn, ParameterServiceDyn};
use crate::error::AppError;

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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl TaskVersion {
    pub async fn parameters<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Parameter>> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        parameter_service.find_by_task_version_id(self.id).await
    }

    pub async fn messages<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Message>> {
        let message_service = ctx
            .data::<MessageServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        message_service.find_by_task_version_id(self.id).await
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
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString)]
#[graphql(remote = "entity::sea_orm_active_enums::TaskVersionStatus")]
pub enum TaskVersionStatus {
    #[strum(serialize = "DRAFT", serialize = "DRAFT")]
    Draft,
    #[strum(serialize = "RELEASED", serialize = "RELEASED")]
    Released,
}
