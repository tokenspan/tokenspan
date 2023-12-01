use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::{Error, Result};
use openai_api_rust::Role;
use serde::{Deserialize, Serialize};

use crate::api::models::UserId;
use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntity {
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
pub struct UserCreateEntity {
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateEntity {
    pub email: Option<String>,
    pub username: Option<String>,
}

impl Repository<UserEntity> {
    pub async fn create(&self, input: UserCreateEntity) -> Result<UserEntity> {
        let user = UserEntity {
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
            .map(|id| UserId::from(id))
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn find_by_email(&self, email: String) -> Result<Option<UserEntity>> {
        self.collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await
    }

    pub async fn update_by_id(
        &self,
        id: UserId,
        doc: UserUpdateEntity,
    ) -> Result<Option<UserEntity>> {
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

    pub async fn find_by_username(&self, username: String) -> Result<Option<UserEntity>> {
        self.collection
            .find_one(
                doc! {
                    "username": username
                },
                None,
            )
            .await
    }
}
