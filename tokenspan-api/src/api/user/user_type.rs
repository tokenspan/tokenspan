use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Role {
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "USER")]
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            Role::Admin => "ADMIN",
            Role::User => "USER",
        };
        write!(f, "{}", role)
    }
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ADMIN" => Role::Admin,
            "USER" => Role::User,
            _ => Role::User,
        }
    }
}
