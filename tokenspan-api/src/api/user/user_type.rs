use std::fmt::Display;
use async_graphql::{Scalar, ScalarType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    #[serde(rename = "Admin")]
    Admin,
    #[serde(rename = "User")]
    User,
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Admin" => Self::Admin,
            "User" => Self::User,
            _ => panic!("invalid role"),
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Admin => "Admin".to_string(),
            Self::User => "User".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[Scalar]
impl ScalarType for Role {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = value {
            let value = match value.as_str() {
                "Admin" => Self::Admin,
                "User" => Self::User,
                _ => return Err(async_graphql::InputValueError::custom("invalid role")),
            };

            Ok(value)
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.to_string())
    }
}
