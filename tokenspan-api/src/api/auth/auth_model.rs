use std::fmt::Display;
use std::str::FromStr;

use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::api::models::{User, UserId};
use crate::prisma::Role as PrismaRole;

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

#[derive(Enum, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Role {
    Admin,
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match *self {
            Self::User => "USER".to_string(),
            Self::Admin => "ADMIN".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN" => Ok(Role::Admin),
            "USER" => Ok(Role::User),
            _ => Err(()),
        }
    }
}

impl From<PrismaRole> for Role {
    fn from(value: PrismaRole) -> Self {
        match value {
            PrismaRole::Admin => Role::Admin,
            PrismaRole::User => Role::User,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParsedToken {
    pub role: Role,
    pub user_id: UserId,
}
