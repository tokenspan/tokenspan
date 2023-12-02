use std::fmt::Debug;

use bson::doc;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::api::models::{UserId, ViewId};
use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewEntity {
    #[serde(rename = "_id")]
    pub id: ViewId,
    pub owner_id: UserId,
    pub name: String,
    pub config: Option<serde_json::Value>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewCreateEntity {
    pub owner_id: UserId,
    pub name: String,
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewUpdateEntity {
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
}

impl Repository<ViewEntity> {
    pub async fn create(&self, doc: ViewCreateEntity) -> Result<ViewEntity> {
        let doc = ViewEntity {
            id: ViewId::new(),
            owner_id: doc.owner_id,
            name: doc.name,
            config: doc.config,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(ViewId::from)
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn update_by_id(
        &self,
        id: ViewId,
        doc: ViewUpdateEntity,
    ) -> Result<Option<ViewEntity>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };
        let config = doc
            .config
            .and_then(|config| bson::ser::to_bson(&config).ok());
        let update = doc! {
            "$set": {
                "name": doc.name,
                "config": config,
                "updated_at": Utc::now(),
            }
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }

    pub async fn find_by_owner_id(&self, owner_id: UserId) -> Result<Vec<ViewEntity>> {
        let filter = doc! {
            "owner_id": ObjectId::from(owner_id),
        };

        let cursor = self.collection.find(filter, None).await?;

        cursor.try_collect().await
    }
}
