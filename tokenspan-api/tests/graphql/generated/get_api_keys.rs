#![allow(clippy::all, warnings)]
pub struct GetApiKeysQuery;
pub mod get_api_keys_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetApiKeysQuery";
    pub const QUERY : & str = "query GetApiKeysQuery {\n  apiKeys {\n    nodes {\n      id\n      name\n      ownerId\n      providerId\n      createdAt\n      updatedAt\n      provider {\n        id\n        name\n        slug\n        createdAt\n        updatedAt\n      }\n      owner {\n        id\n        email\n        username\n        role\n      }\n    }\n    totalNodes\n    pageInfo {\n      startCursor\n      endCursor\n      hasNextPage\n      hasPreviousPage\n    }\n  }\n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type NaiveDateTime = crate::graphql::NaiveDateTime;
    type UUID = crate::graphql::UUID;
    #[derive(Debug, PartialEq)]
    pub enum UserRole {
        ADMIN,
        USER,
        Other(String),
    }
    impl ::serde::Serialize for UserRole {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                UserRole::ADMIN => "ADMIN",
                UserRole::USER => "USER",
                UserRole::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserRole {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "ADMIN" => Ok(UserRole::ADMIN),
                "USER" => Ok(UserRole::USER),
                _ => Ok(UserRole::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "apiKeys")]
        pub api_keys: GetApiKeysQueryApiKeys,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeysQueryApiKeys {
        pub nodes: Vec<GetApiKeysQueryApiKeysNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: GetApiKeysQueryApiKeysPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeysQueryApiKeysNodes {
        pub id: UUID,
        pub name: String,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        #[serde(rename = "providerId")]
        pub provider_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
        pub provider: Option<GetApiKeysQueryApiKeysNodesProvider>,
        pub owner: Option<GetApiKeysQueryApiKeysNodesOwner>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeysQueryApiKeysNodesProvider {
        pub id: UUID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeysQueryApiKeysNodesOwner {
        pub id: UUID,
        pub email: String,
        pub username: String,
        pub role: UserRole,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeysQueryApiKeysPageInfo {
        #[serde(rename = "startCursor")]
        pub start_cursor: Option<String>,
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "hasPreviousPage")]
        pub has_previous_page: Boolean,
    }
}
impl graphql_client::GraphQLQuery for GetApiKeysQuery {
    type Variables = get_api_keys_query::Variables;
    type ResponseData = get_api_keys_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_api_keys_query::QUERY,
            operation_name: get_api_keys_query::OPERATION_NAME,
        }
    }
}
