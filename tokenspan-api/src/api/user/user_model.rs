use std::fmt::Display;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject, Value};

use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::Cursor;

use crate::api::models::Role;
use crate::prisma::user;

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserId(pub String);

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
