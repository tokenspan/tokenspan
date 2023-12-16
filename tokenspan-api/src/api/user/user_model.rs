use async_graphql::{Enum, SimpleObject};
use sea_orm::prelude::Uuid;
use strum_macros::{Display, EnumString};

pub type UserId = Uuid;

#[derive(SimpleObject, Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    #[graphql(skip)]
    pub salt: String,
    pub role: UserRole,
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
        }
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Display)]
#[graphql(remote = "entity::sea_orm_active_enums::UserRole")]
pub enum UserRole {
    Admin,
    User,
}
