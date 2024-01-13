#![allow(clippy::all, warnings)]
pub struct GetApiKeysQuery;
pub mod get_api_keys_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetApiKeysQuery";
    pub const QUERY : & str = "query GetApiKeysQuery($args: ApiKeyArgs!) {\n  apiKeys(args: $args) {\n    nodes {\n      id\n      name\n      providerId\n      createdAt\n    }\n    totalNodes\n    pageInfo {\n      startCursor\n      endCursor\n      hasNextPage\n      hasPreviousPage\n    }\n  }\n}" ;
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
    type Cursor = crate::graphql::Cursor;
    type NaiveDateTime = crate::graphql::NaiveDateTime;
    type UUID = crate::graphql::UUID;
    #[derive(Serialize)]
    pub struct ApiKeyArgs {
        pub after: Option<Cursor>,
        pub before: Option<Cursor>,
        pub first: Option<Int>,
        pub last: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub args: ApiKeyArgs,
    }
    impl Variables {}
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
        #[serde(rename = "providerId")]
        pub provider_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
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
