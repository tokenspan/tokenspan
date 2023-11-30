use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use bson::{doc};
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::error::{Error, Result};
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokenspan_utils::pagination::{Cursor, CursorExt, Pagination};

use crate::api::models::{UserId, ViewId};
use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewDoc {
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
pub struct ViewCreateDoc {
    pub owner_id: UserId,
    pub name: String,
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewUpdateDoc {
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
}

impl Repository<ViewDoc> {
    pub async fn create(&self, doc: ViewCreateDoc) -> Result<ViewDoc> {
        let doc = ViewDoc {
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
            .map(|id| id.into())
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn find_by_id(&self, id: ViewId) -> Result<Option<ViewDoc>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };

        self.collection.find_one(filter, None).await
    }

    pub async fn paginate<T: CursorExt<Cursor> + From<ViewDoc> + Debug>(
        &self,
        take: Option<i64>,
        before: Option<Cursor>,
        after: Option<Cursor>,
    ) -> Result<Pagination<Cursor, T>> {
        let take = take.unwrap_or(1);
        let limit = take
            + if after.is_some() || before.is_some() {
                2
            } else {
                1
            };

        let filter = after
            .clone()
            .map(|cursor| doc! { "_id": { "$lte": ObjectId::parse_str(cursor.id).unwrap() } })
            .or_else(|| {
                before.clone().map(
                    |cursor| doc! { "_id": { "$gte": ObjectId::parse_str(cursor.id).unwrap() } },
                )
            })
            .unwrap_or(doc! {});

        let options = FindOptions::builder()
            .sort(doc! {
                "_id": -1
            })
            .limit(limit)
            .build();

        let items = self
            .collection
            .find(filter, Some(options))
            .await?
            .try_collect::<Vec<ViewDoc>>()
            .await?
            .into_iter()
            .map(|doc| doc.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, before, after, take))
    }

    pub async fn find_many_by_ids(&self, ids: Vec<ViewId>) -> Result<Vec<ViewDoc>> {
        let ids = ids
            .into_iter()
            .map(|id| id.into())
            .collect::<Vec<ObjectId>>();

        let cursor = self
            .collection
            .find(
                doc! {
                    "_id": {
                        "$in": ids
                    }
                },
                None,
            )
            .await?;

        cursor.try_collect().await
    }

    pub async fn find_by_owner_id(&self, owner_id: UserId) -> Result<Vec<ViewDoc>> {
        let filter = doc! {
            "owner_id": ObjectId::from(owner_id),
        };

        let cursor = self.collection.find(filter, None).await?;

        cursor.try_collect().await
    }

    pub async fn update(&self, id: ViewId, doc: ViewUpdateDoc) -> Result<Option<ViewDoc>> {
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

    pub async fn delete(&self, id: ViewId) -> Result<Option<ViewDoc>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };

        self.collection.find_one_and_delete(filter, None).await
    }

    pub async fn count(&self) -> Result<u64> {
        let filter = doc! {};

        self.collection.count_documents(filter, None).await
    }
}
