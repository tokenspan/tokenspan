use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::iter::Iterator;

use crate::api::models::UserId;
use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDoc {
    #[serde(rename = "_id")]
    pub id: UserId,
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub role: Role,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateDoc {
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateDoc {
    pub email: Option<String>,
    pub username: Option<String>,
}

impl Repository<UserDoc> {
    pub async fn create(&self, input: UserCreateDoc) -> Result<UserDoc> {
        let user = UserDoc {
            id: UserId::new(),
            email: input.email,
            username: input.username,
            password: input.password,
            salt: input.salt,
            role: Role::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(user, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(|id| id.into())
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn find_by_id(&self, id: UserId) -> Result<Option<UserDoc>> {
        self.collection
            .find_one(
                doc! {
                    "_id": ObjectId::from(id)
                },
                None,
            )
            .await
    }

    pub async fn find_by_email(&self, email: String) -> Result<Option<UserDoc>> {
        self.collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await
    }

    pub async fn find_by_username(&self, username: String) -> Result<Option<UserDoc>> {
        self.collection
            .find_one(
                doc! {
                    "username": username
                },
                None,
            )
            .await
    }

    pub async fn find_many_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<UserDoc>> {
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

    pub async fn update_by_id(&self, id: UserId, doc: UserUpdateDoc) -> Result<Option<UserDoc>> {
        self.collection
            .find_one_and_update(
                doc! {
                    "_id": ObjectId::from(id)
                },
                doc! {
                    "$set": {
                        "email": doc.email,
                        "username": doc.username,
                        "updated_at": Utc::now()
                    }
                },
                None,
            )
            .await
    }

    pub async fn delete_by_id(&self, id: UserId) -> Result<Option<UserDoc>> {
        self.collection
            .find_one_and_delete(
                doc! {
                    "_id": ObjectId::from(id)
                },
                None,
            )
            .await
    }
}
