use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use rabbit_macros::Model;
use rabbit_orm::pagination::{Cursor, CursorExt};
use serde::Deserialize;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug, Model)]
#[rabbit(name = "users")]
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

impl CursorExt<Cursor> for User {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Display, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[strum(serialize = "admin")]
    Admin,
    #[strum(serialize = "user")]
    User,
}
