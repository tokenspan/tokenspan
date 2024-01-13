#![allow(clippy::all, warnings)]
pub struct GetApiKeyQuery;
pub mod get_api_key_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetApiKeyQuery";
    pub const QUERY : & str = "query GetApiKeyQuery($id: UUID!) {\n  apiKey(id: $id) {\n    id\n    name\n    ownerId\n    providerId\n    createdAt\n    updatedAt\n    provider {\n      id\n      name\n      slug\n    }\n    owner {\n      id\n      username\n    }\n  }\n}\n" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        pub id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "apiKey")]
        pub api_key: Option<GetApiKeyQueryApiKey>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeyQueryApiKey {
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
        pub provider: Option<GetApiKeyQueryApiKeyProvider>,
        pub owner: Option<GetApiKeyQueryApiKeyOwner>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeyQueryApiKeyProvider {
        pub id: UUID,
        pub name: String,
        pub slug: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetApiKeyQueryApiKeyOwner {
        pub id: UUID,
        pub username: String,
    }
}
impl graphql_client::GraphQLQuery for GetApiKeyQuery {
    type Variables = get_api_key_query::Variables;
    type ResponseData = get_api_key_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_api_key_query::QUERY,
            operation_name: get_api_key_query::OPERATION_NAME,
        }
    }
}
