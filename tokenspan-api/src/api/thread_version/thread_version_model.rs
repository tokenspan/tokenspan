use async_graphql::{ComplexObject, Enum, SimpleObject};
use async_graphql::{Context, Result};
use chrono::NaiveDateTime;
use dojo_macros::{Model, Type};
use serde::Deserialize;
use strum_macros::EnumString;
use uuid::Uuid;

use crate::api::models::{Message, Parameter, Thread};
use crate::api::services::{MessageServiceDyn, ParameterServiceDyn, ThreadServiceDyn};
use crate::error::AppError;

#[derive(SimpleObject, Clone, Debug, Model)]
#[graphql(complex)]
#[dojo(name = "thread_versions", sort_keys = ["created_at", "id"])]
pub struct ThreadVersion {
    pub id: Uuid,
    pub semver: String,
    pub version: i32,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub status: ThreadVersionStatus,
    pub thread_id: Uuid,
    pub owner_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl ThreadVersion {
    pub async fn thread<'a>(&self, ctx: &Context<'a>) -> Result<Option<Thread>> {
        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread = thread_service.find_by_id(&self.thread_id).await?;

        Ok(thread)
    }

    pub async fn parameters<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Parameter>> {
        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameters = parameter_service
            .find_by_thread_version_id(&self.id)
            .await?;

        Ok(parameters)
    }

    pub async fn messages<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Message>> {
        let message_service = ctx
            .data::<MessageServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let messages = message_service.find_by_thread_version_id(&self.id).await?;

        Ok(messages)
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Deserialize, Type)]
#[dojo(name = "thread_version_status", rename_all = "lowercase")]
pub enum ThreadVersionStatus {
    #[strum(serialize = "draft")]
    #[serde(rename = "draft")]
    Draft,
    #[strum(serialize = "published")]
    #[serde(rename = "published")]
    Published,
}
