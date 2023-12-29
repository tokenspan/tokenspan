use async_graphql::{ComplexObject, Enum, SimpleObject};
use async_graphql::{Context, Result};
use chrono::NaiveDateTime;
use dojo_macros::{Model, Type};
use dojo_orm::pagination::{Cursor, CursorExt};
use serde::Deserialize;
use strum_macros::EnumString;
use uuid::Uuid;

use crate::api::models::{Message, Parameter, Task};
use crate::api::services::{ParameterServiceDyn, TaskServiceDyn};
use crate::error::AppError;

#[derive(SimpleObject, Clone, Model)]
#[graphql(complex)]
#[dojo(name = "task_versions")]
pub struct TaskVersion {
    pub id: Uuid,
    pub semver: String,
    pub version: i32,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub status: TaskVersionStatus,
    pub task_id: Uuid,
    pub owner_id: Uuid,
    #[dojo(embedded)]
    pub messages: Vec<Message>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl TaskVersion {
    pub async fn task<'a>(&self, ctx: &Context<'a>) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.find_by_id(self.task_id).await?;

        Ok(task)
    }

    pub async fn parameters<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Parameter>> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameters = parameter_service.find_by_task_version_id(self.id).await?;

        Ok(parameters)
    }
}

impl CursorExt<Cursor> for TaskVersion {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Deserialize, Type)]
#[dojo(name = "task_version_status", rename_all = "lowercase")]
pub enum TaskVersionStatus {
    #[strum(serialize = "draft")]
    #[serde(rename = "published")]
    Draft,
    #[strum(serialize = "published")]
    #[serde(rename = "published")]
    Published,
}
