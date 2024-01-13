#![allow(clippy::all, warnings)]
pub struct GetThreadVersionsQuery;
pub mod get_thread_versions_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetThreadVersionsQuery";
    pub const QUERY : & str = "query GetThreadVersionsQuery($args: ThreadVersionArgs!) {\n  threadVersions(args: $args) {\n    nodes {\n      id\n      threadId\n      ownerId\n      createdAt\n    }\n    totalNodes\n    pageInfo {\n      hasPreviousPage\n      hasNextPage\n      startCursor\n      endCursor\n    }\n  }\n}" ;
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
    pub struct ThreadVersionArgs {
        pub after: Option<Cursor>,
        pub before: Option<Cursor>,
        pub first: Option<Int>,
        pub last: Option<Int>,
        #[serde(rename = "threadId")]
        pub thread_id: UUID,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub args: ThreadVersionArgs,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "threadVersions")]
        pub thread_versions: GetThreadVersionsQueryThreadVersions,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersions {
        pub nodes: Vec<GetThreadVersionsQueryThreadVersionsNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: GetThreadVersionsQueryThreadVersionsPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersionsNodes {
        pub id: UUID,
        #[serde(rename = "threadId")]
        pub thread_id: UUID,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersionsPageInfo {
        #[serde(rename = "hasPreviousPage")]
        pub has_previous_page: Boolean,
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "startCursor")]
        pub start_cursor: Option<String>,
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for GetThreadVersionsQuery {
    type Variables = get_thread_versions_query::Variables;
    type ResponseData = get_thread_versions_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_thread_versions_query::QUERY,
            operation_name: get_thread_versions_query::OPERATION_NAME,
        }
    }
}
