use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::models::{User, UserRole};

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
    pub sub: Uuid,
    pub exp: i64,
}

#[derive(Clone, Debug)]
pub struct ParsedToken {
    pub role: UserRole,
    pub user_id: Uuid,
}
