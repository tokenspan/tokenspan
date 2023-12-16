use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use serde::Serialize;

use tokenspan_extra::pagination::{Cursor, CursorExt};

use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::error::AppError;

pub type TaskId = Uuid;

#[derive(SimpleObject, Clone, Serialize)]
#[graphql(complex)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Task {
    pub async fn version<'a>(
        &self,
        ctx: &Context<'a>,
        version: Option<String>,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version = if let Some(version) = version {
            task_version_service
                .find_by_version(self.id.clone(), version)
                .await?
        } else {
            task_version_service.find_latest(self.id.clone()).await?
        };

        Ok(task_version)
    }
}

impl CursorExt<Cursor> for Task {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::task::Model> for Task {
    fn from(input: entity::task::Model) -> Self {
        Self {
            id: input.id,
            name: input.name,
            slug: input.slug,
            created_at: input.created_at,
            updated_at: input.updated_at,
        }
    }
}
