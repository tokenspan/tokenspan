#![allow(clippy::all, warnings)]
pub struct GetFunctionsQuery;
pub mod get_functions_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetFunctionsQuery";
    pub const QUERY : & str = "query GetFunctionsQuery {\n  functions {\n    totalNodes\n    pageInfo {\n      hasPreviousPage\n      hasNextPage\n      startCursor\n      endCursor\n    }\n    nodes {\n      id\n      ownerId\n      name\n      description\n      parameters\n      response\n      createdAt\n      updatedAt\n    }\n  }\n}" ;
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
    type JSON = crate::graphql::JSON;
    type NaiveDateTime = crate::graphql::NaiveDateTime;
    type UUID = crate::graphql::UUID;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub functions: GetFunctionsQueryFunctions,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetFunctionsQueryFunctions {
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: GetFunctionsQueryFunctionsPageInfo,
        pub nodes: Vec<GetFunctionsQueryFunctionsNodes>,
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
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetFunctionsQueryFunctionsNodes {
        pub id: UUID,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        pub name: String,
        pub description: String,
        pub parameters: JSON,
        pub response: Option<JSON>,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
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
