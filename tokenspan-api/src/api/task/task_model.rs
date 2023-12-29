use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::NaiveDateTime;
use dojo_macros::Model;
use serde::Serialize;

use dojo_orm::pagination::{Cursor, CursorExt};
use uuid::Uuid;

use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::error::AppError;

#[derive(SimpleObject, Clone, Serialize, Model)]
#[graphql(complex)]
#[dojo(name = "tasks")]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub owner_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Task {
    pub async fn version<'a>(
        &self,
        ctx: &Context<'a>,
        semver: Option<String>,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version = if let Some(semver) = semver {
            task_version_service
                .find_by_semver(self.id.clone(), semver)
                .await?
        } else {
            task_version_service.find_latest(self.id.clone()).await?
        };

        Ok(task_version)
    }
}

impl CursorExt<Cursor> for Task {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}
