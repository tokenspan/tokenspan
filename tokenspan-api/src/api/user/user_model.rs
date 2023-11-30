use std::fmt::Display;


use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject, Value};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::Cursor;

use crate::api::models::Role;
use crate::prisma::user;

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

impl From<user::Data> for User {
    fn from(value: user::Data) -> Self {
        Self {
            id: UserId::try_from(value.id).unwrap(),
            email: value.email,
            password: value.password,
            salt: value.salt,
            role: value.role.into(),
        }
    }
}

impl From<super::user_repository::UserDoc> for User {
    fn from(value: super::user_repository::UserDoc) -> Self {
        Self {
            id: UserId::try_from(value.id.to_string()).unwrap(),
            email: value.email,
            password: value.password,
            salt: value.salt,
            role: Role::User,
        }
    }
}
