use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use dojo_macros::{Model, Type};
use serde::Deserialize;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug, Model)]
#[dojo(name = "users", sort_keys = ["created_at", "id"])]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    #[graphql(skip)]
    pub salt: String,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, Display, EnumString, Deserialize, Type)]
#[dojo(name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[strum(serialize = "admin")]
    #[serde(rename = "admin")]
    Admin,
    #[strum(serialize = "user")]
    #[serde(rename = "user")]
    User,
}
