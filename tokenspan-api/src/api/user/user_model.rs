use std::fmt::Display;
use std::str::FromStr;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject, Value};
use bson::oid::ObjectId;

use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::Cursor;

use crate::api::models::Role;
use crate::prisma::user;

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserId(pub String);

impl From<UserId> for ObjectId {
    fn from(value: UserId) -> Self {
        ObjectId::from_str(&value.0).unwrap()
    }
}

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
            id: UserId(value.id),
            email: value.email,
            password: value.password,
            salt: value.salt,
            role: value.role.into(),
        }
    }
}

impl From<super::user_repository::User> for User {
    fn from(value: crate::api::repositories::User) -> Self {
        Self {
            id: UserId(value.id.to_string()),
            email: value.email,
            password: value.password,
            salt: value.salt,
            role: Role::User,
        }
    }
}
