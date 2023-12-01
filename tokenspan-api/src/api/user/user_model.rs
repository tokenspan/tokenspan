use std::fmt::Display;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use tokenspan_macros::ID;

use crate::api::models::Role;

#[derive(ID, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserId(ObjectId);

#[derive(SimpleObject, Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    #[graphql(skip)]
    pub salt: String,
    pub role: Role,
}

impl From<super::user_repository::UserEntity> for User {
    fn from(value: super::user_repository::UserEntity) -> Self {
        Self {
            id: value.id,
            email: value.email,
            password: value.password,
            salt: value.salt,
            role: Role::User,
        }
    }
}
