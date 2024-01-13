#![allow(clippy::all, warnings)]
pub struct GetThreadsQuery;
pub mod get_threads_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetThreadsQuery";
    pub const QUERY : & str = "query GetThreadsQuery($args: ThreadArgs!) {\n  threads(args: $args) {\n    nodes {\n      id\n      name\n      slug\n      ownerId\n      createdAt\n      updatedAt\n    }\n    totalNodes\n    pageInfo {\n      hasPreviousPage\n      hasNextPage\n      startCursor\n      endCursor\n    }\n  }\n}" ;
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
    pub struct ThreadArgs {
        pub after: Option<Cursor>,
        pub before: Option<Cursor>,
        pub filter: Option<ThreadFilter>,
        pub first: Option<Int>,
        pub last: Option<Int>,
        pub sort: Option<ThreadSort>,
    }
    #[derive(Serialize)]
    pub struct ThreadFilter {
        pub name: Option<String>,
        #[serde(rename = "ownerId")]
        pub owner_id: Option<UUID>,
        pub slug: Option<String>,
    }
    #[derive(Serialize)]
    pub struct ThreadSort {
        #[serde(rename = "createdAt")]
        pub created_at: Option<NaiveDateTime>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub args: ThreadArgs,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub threads: GetThreadsQueryThreads,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadsQueryThreads {
        pub nodes: Vec<GetThreadsQueryThreadsNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: GetThreadsQueryThreadsPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadsQueryThreadsNodes {
        pub id: UUID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadsQueryThreadsPageInfo {
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
impl graphql_client::GraphQLQuery for GetThreadsQuery {
    type Variables = get_threads_query::Variables;
    type ResponseData = get_threads_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_threads_query::QUERY,
            operation_name: get_threads_query::OPERATION_NAME,
        }
    }
}
