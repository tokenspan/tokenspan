#![allow(clippy::all, warnings)]
pub struct GetProvidersQuery;
pub mod get_providers_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetProvidersQuery";
    pub const QUERY : & str = "query GetProvidersQuery($args: ProviderArgs!) {\n  providers(args: $args) {\n    nodes {\n      id\n      name\n      createdAt\n    }\n    totalNodes\n    pageInfo {\n      endCursor\n      startCursor\n      hasNextPage\n      hasPreviousPage\n    }\n  }\n}" ;
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
    pub struct ProviderArgs {
        pub after: Option<Cursor>,
        pub before: Option<Cursor>,
        pub first: Option<Int>,
        pub last: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub args: ProviderArgs,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub providers: GetProvidersQueryProviders,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetProvidersQueryProviders {
        pub nodes: Vec<GetProvidersQueryProvidersNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: GetProvidersQueryProvidersPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetProvidersQueryProvidersNodes {
        pub id: UUID,
        pub name: String,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetProvidersQueryProvidersPageInfo {
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
        #[serde(rename = "startCursor")]
        pub start_cursor: Option<String>,
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "hasPreviousPage")]
        pub has_previous_page: Boolean,
    }
}
impl graphql_client::GraphQLQuery for GetProvidersQuery {
    type Variables = get_providers_query::Variables;
    type ResponseData = get_providers_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_providers_query::QUERY,
            operation_name: get_providers_query::OPERATION_NAME,
        }
    }
}
