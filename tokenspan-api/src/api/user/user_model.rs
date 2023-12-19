use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use sqlx::FromRow;
use strum_macros::{Display, EnumString};

use tokenspan_extra::pagination::{Cursor, CursorExt};
use tokenspan_macros::FieldNames;

pub trait Updater {
    fn set_id(&mut self, id: Option<Uuid>);
}

#[derive(SimpleObject, Clone, Debug, FromRow, FieldNames)]
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
        self.created_at.into()
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Display, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
pub enum UserRole {
    #[strum(serialize = "ADMIN")]
    Admin,
    #[strum(serialize = "USER")]
    User,
}
