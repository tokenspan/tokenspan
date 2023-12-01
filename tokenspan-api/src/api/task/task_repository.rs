use crate::api::models::{TaskId, UserId};
use crate::repository::Repository;
use bson::doc;
use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskEntity {
    pub id: TaskId,
    pub owner_id: UserId,
    pub name: String,
    pub slug: String,
    pub private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreateEntity {
    pub owner_id: UserId,
    pub name: String,
    pub slug: String,
    pub private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdateEntity {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub private: Option<bool>,
}

impl Repository<TaskEntity> {
    pub async fn create(&self, doc: TaskCreateEntity) -> Result<TaskEntity> {
        let doc = TaskEntity {
            id: TaskId::new(),
            owner_id: doc.owner_id,
            name: doc.name,
            slug: doc.slug,
            private: doc.private,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(|id| TaskId::from(id))
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn update_by_id(
        &self,
        id: TaskId,
        doc: TaskUpdateEntity,
    ) -> Result<Option<TaskEntity>> {
        let filter = doc! {
            "_id": id,
        };
        let update = doc! {
            "$set": {
                "updated_at": Utc::now(),
                "name": doc.name,
                "slug": doc.slug,
                "private": doc.private,
            },
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }
}
