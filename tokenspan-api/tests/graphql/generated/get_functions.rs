#![allow(clippy::all, warnings)]
pub struct GetFunctionsQuery;
pub mod get_functions_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetFunctionsQuery";
    pub const QUERY : & str = "query GetFunctionsQuery($args: FunctionArgs!) {\n  functions(args: $args) {\n    nodes {\n      id\n      name\n      ownerId\n      createdAt\n    }\n    totalNodes\n    pageInfo {\n      hasPreviousPage\n      hasNextPage\n      startCursor\n      endCursor\n    }\n  }\n}" ;
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
    pub struct FunctionArgs {
        pub after: Option<Cursor>,
        pub before: Option<Cursor>,
        pub first: Option<Int>,
        pub last: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub args: FunctionArgs,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub functions: GetFunctionsQueryFunctions,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetFunctionsQueryFunctions {
        pub nodes: Vec<GetFunctionsQueryFunctionsNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: GetFunctionsQueryFunctionsPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetFunctionsQueryFunctionsNodes {
        pub id: UUID,
        pub name: String,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetFunctionsQueryFunctionsPageInfo {
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
impl graphql_client::GraphQLQuery for GetFunctionsQuery {
    type Variables = get_functions_query::Variables;
    type ResponseData = get_functions_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_functions_query::QUERY,
            operation_name: get_functions_query::OPERATION_NAME,
        }
    }
}
