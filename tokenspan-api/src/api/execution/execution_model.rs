use async_graphql::{ComplexObject, Enum, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::api::models::{Message, Parameter};
use tokenspan_extra::pagination::{Cursor, CursorExt};

#[derive(SimpleObject, InputObject, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

#[derive(SimpleObject, InputObject, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Elapsed {
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

#[derive(SimpleObject, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
pub struct Execution {
    pub id: Uuid,
    pub task_version_id: Uuid,
    pub executed_by_id: Uuid,
    #[graphql(skip)]
    pub elapsed: serde_json::Value,
    #[graphql(skip)]
    pub messages: serde_json::Value,
    #[graphql(skip)]
    pub parameters: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    #[graphql(skip)]
    pub usage: Option<serde_json::Value>,
    pub status: ExecutionStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Execution {
    pub async fn parameters(&self) -> async_graphql::Result<Vec<Parameter>> {
        let parameters: Vec<Parameter> = serde_json::from_value(self.parameters.clone())?;

        Ok(parameters)
    }

    pub async fn messages(&self) -> async_graphql::Result<Vec<Message>> {
        let messages: Vec<Message> = serde_json::from_value(self.messages.clone())?;

        Ok(messages)
    }

    pub async fn elapsed(&self) -> async_graphql::Result<Elapsed> {
        let elapsed: Elapsed = serde_json::from_value(self.elapsed.clone())?;

        Ok(elapsed)
    }

    pub async fn usage(&self) -> async_graphql::Result<Option<Usage>> {
        if let Some(usage) = &self.usage {
            let usage: Usage = serde_json::from_value(usage.clone())?;

            return Ok(Some(usage));
        }

        Ok(None)
    }
}

impl CursorExt<Cursor> for Execution {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::execution::Model> for Execution {
    fn from(value: entity::execution::Model) -> Self {
        Self {
            id: value.id.into(),
            task_version_id: value.task_version_id.into(),
            executed_by_id: value.executor_id.into(),
            elapsed: value.elapsed,
            usage: value.usage,
            messages: value.messages,
            parameters: value.parameter,
            output: value.output,
            error: value.error,
            status: ExecutionStatus::from(value.status),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Display, Serialize)]
#[graphql(remote = "entity::sea_orm_active_enums::ExecutionStatus")]
pub enum ExecutionStatus {
    #[strum(serialize = "SUCCESS", serialize = "SUCCESS")]
    Success,
    #[strum(serialize = "FAILURE", serialize = "FAILURE")]
    Failure,
}
