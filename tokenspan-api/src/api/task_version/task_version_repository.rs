use std::collections::HashMap;
use std::fmt::Display;

use async_graphql::Enum;
use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use bson::{doc, Bson};
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::error::{Error, Result};
use mongodb::options::FindOneOptions;
use serde::{Deserialize, Serialize};

use crate::api::models::{TaskId, TaskVersionId, UserId};
use crate::api::repositories::ParameterEntity;
use crate::prompt::{ChatMessage, RawChatMessage};
use crate::repository::Repository;

#[derive(Deserialize, Serialize, Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum TaskVersionStatus {
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "PUBLISHED")]
    Published,
    #[serde(rename = "ARCHIVED")]
    Archived,
}

impl Display for TaskVersionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskVersionStatus::Draft => write!(f, "DRAFT"),
            TaskVersionStatus::Published => write!(f, "PUBLISHED"),
            TaskVersionStatus::Archived => write!(f, "ARCHIVED"),
        }
    }
}

impl From<TaskVersionStatus> for Bson {
    fn from(value: TaskVersionStatus) -> Self {
        Bson::String(value.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskVersionEntity {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub task_id: TaskId,
    pub owner_id: UserId,
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub parameters: Vec<ParameterEntity>,
    pub messages: Vec<ChatMessage>,
    pub raw_messages: Vec<RawChatMessage>,
    pub variables: HashMap<String, String>,
    pub status: TaskVersionStatus,
    pub release_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskVersionCreateEntity {
    pub task_id: TaskId,
    pub owner_id: UserId,
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub parameters: Vec<ParameterEntity>,
    pub messages: Vec<ChatMessage>,
    pub raw_messages: Vec<RawChatMessage>,
    pub variables: HashMap<String, String>,
    pub status: TaskVersionStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskVersionUpdateEntity {
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Option<Vec<ChatMessage>>,
    pub raw_messages: Option<Vec<String>>,
    pub variables: HashMap<String, String>,
    pub status: Option<TaskVersionStatus>,
}

impl Repository<TaskVersionEntity> {
    pub async fn create(&self, doc: TaskVersionCreateEntity) -> Result<TaskVersionEntity> {
        let doc = TaskVersionEntity {
            id: ObjectId::new(),
            task_id: doc.task_id,
            owner_id: doc.owner_id,
            version: doc.version,
            release_note: doc.release_note,
            description: doc.description,
            document: doc.document,
            parameters: doc.parameters,
            messages: doc.messages,
            raw_messages: doc.raw_messages,
            variables: doc.variables,
            status: doc.status,
            release_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(TaskVersionId::from)
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("task version not found"))
    }

    pub async fn update_by_id(
        &self,
        id: TaskVersionId,
        doc: TaskVersionUpdateEntity,
    ) -> Result<Option<TaskVersionEntity>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };
        let messages = doc
            .messages
            .and_then(|config| bson::ser::to_bson(&config).ok());

        let update = doc! {
            "$set": {
                "updatedAt": Utc::now(),
                "releaseNote": doc.release_note,
                "description": doc.description,
                "document": doc.document,
                "messages": messages,
                // "status": doc.status,
            }
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }

    pub async fn find_by_task_id(&self, task_id: TaskId) -> Result<Vec<TaskVersionEntity>> {
        let filter = doc! {
            "taskId": ObjectId::from(task_id),
        };

        let cursor = self.collection.find(filter, None).await?;

        cursor.try_collect().await
    }

    pub async fn find_by_version(
        &self,
        task_id: TaskId,
        version: String,
    ) -> Result<Option<TaskVersionEntity>> {
        let task_id = ObjectId::from(task_id);
        let filter = doc! {
            "taskId": task_id,
            "version": version,
        };

        self.collection.find_one(filter, None).await
    }

    pub async fn find_latest(&self, task_id: TaskId) -> Result<Option<TaskVersionEntity>> {
        let task_id = ObjectId::from(task_id);
        let filter = doc! {
            "taskId": task_id,
            "status": TaskVersionStatus::Published,
        };
        let options = FindOneOptions::builder()
            .sort(doc! {
                "version": -1,
            })
            .build();

        self.collection.find_one(filter, Some(options)).await
    }
}
