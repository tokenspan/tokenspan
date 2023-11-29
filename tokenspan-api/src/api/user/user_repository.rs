use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
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
pub struct UserCreateInput {
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateInput {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug)]
pub struct UserRepository {
    pub collection_name: String,
    pub collection: mongodb::Collection<User>,
    pub db: mongodb::Database,
}

impl UserRepository {
    pub fn new(db: mongodb::Database) -> Self {
        let collection_name = String::from("users");
        let collection = db.collection(&collection_name);
        UserRepository {
            collection_name,
            collection,
            db,
        }
    }

    pub async fn create(&self, input: UserCreateInput) -> Result<User, mongodb::error::Error> {
        let user = User {
            id: ObjectId::new(),
            email: input.email,
            username: input.username,
            password: input.password,
            salt: String::from(""),
            role: Role::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(user, None).await?;
        let user = self
            .find_by_id(result.inserted_id.as_object_id().unwrap())
            .await?;
        match user {
            Some(user) => Ok(user),
            None => Err(mongodb::error::Error::custom("User not found")),
        }
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<User>, mongodb::error::Error> {
        let user = self
            .collection
            .find_one(
                doc! {
                    "_id": id
                },
                None,
            )
            .await?;
        match user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    pub async fn find_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, mongodb::error::Error> {
        let user = self
            .collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await?;
        match user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    pub async fn find_by_username(
        &self,
        username: String,
    ) -> Result<Option<User>, mongodb::error::Error> {
        let user = self
            .collection
            .find_one(
                doc! {
                    "username": username
                },
                None,
            )
            .await?;
        match user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    pub async fn find_many_by_ids(
        &self,
        ids: Vec<ObjectId>,
    ) -> Result<Vec<User>, mongodb::error::Error> {
        let mut cursor = self
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

        let mut users = vec![];
        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }

        Ok(users)
    }

    pub async fn update_by_id(
        &self,
        id: ObjectId,
        input: UserUpdateInput,
    ) -> Result<Option<User>, mongodb::error::Error> {
        let mut update = doc! {};
        if let Some(email) = input.email {
            update.insert("email", email);
        }
        if let Some(username) = input.username {
            update.insert("username", username);
        }
        if let Some(password) = input.password {
            update.insert("password", password);
        }
        let user = self
            .collection
            .find_one_and_update(
                doc! {
                    "_id": id
                },
                doc! {
                    "$set": update
                },
                None,
            )
            .await?;

        match user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    pub async fn delete_by_id(&self, id: ObjectId) -> Result<Option<User>, mongodb::error::Error> {
        let user = self
            .collection
            .find_one_and_delete(
                doc! {
                    "_id": id
                },
                None,
            )
            .await?;
        match user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }
}
