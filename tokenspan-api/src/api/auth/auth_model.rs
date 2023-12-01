use std::fmt::Display;
use std::str::FromStr;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::api::models::{User, UserId};
use crate::api::repositories::Role;

#[derive(SimpleObject)]
pub struct AuthPayload {
    pub refresh_token: String,
    pub token: String,
    pub user: User,
}

#[derive(SimpleObject)]
pub struct SessionPayload {
    pub user: User,
}

#[derive(SimpleObject)]
pub struct RefreshPayload {
    pub refresh_token: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub role: String,
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub exp: i64,
}

#[derive(Clone, Debug)]
pub struct ParsedToken {
    pub role: Role,
    pub user_id: UserId,
}
