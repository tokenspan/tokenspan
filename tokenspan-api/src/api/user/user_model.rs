use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use strum_macros::{Display, EnumString};

use tokenspan_extra::pagination::{Cursor, CursorExt};

#[derive(SimpleObject, Clone, Debug)]
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
}

impl CursorExt<Cursor> for User {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::user::Model> for User {
    fn from(value: entity::user::Model) -> Self {
        Self {
            id: value.id.into(),
            email: value.email,
            username: value.username,
            password: value.password,
            salt: value.salt,
            role: UserRole::from(value.role),
            created_at: value.created_at,
        }
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Display)]
#[graphql(remote = "entity::sea_orm_active_enums::UserRole")]
pub enum UserRole {
    Admin,
    User,
}
