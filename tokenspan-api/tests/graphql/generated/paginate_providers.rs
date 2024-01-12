#![allow(clippy::all, warnings)]
pub struct PaginateProvidersQuery;
pub mod paginate_providers_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PaginateProvidersQuery";
    pub const QUERY : & str = "query PaginateProvidersQuery($args: ProviderArgs!) {\n  providers(args: $args) {\n    nodes {\n      name\n    }\n    totalNodes\n    pageInfo {\n      endCursor\n      startCursor\n      hasNextPage\n      hasPreviousPage\n    }\n  }\n}" ;
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
        pub providers: PaginateProvidersQueryProviders,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct PaginateProvidersQueryProviders {
        pub nodes: Vec<PaginateProvidersQueryProvidersNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: PaginateProvidersQueryProvidersPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct PaginateProvidersQueryProvidersNodes {
        pub name: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct PaginateProvidersQueryProvidersPageInfo {
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
impl graphql_client::GraphQLQuery for PaginateProvidersQuery {
    type Variables = paginate_providers_query::Variables;
    type ResponseData = paginate_providers_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: paginate_providers_query::QUERY,
            operation_name: paginate_providers_query::OPERATION_NAME,
        }
    }
}
