#![allow(clippy::all, warnings)]
pub struct PaginateModelsQuery;
pub mod paginate_models_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PaginateModelsQuery";
    pub const QUERY : & str = "query PaginateModelsQuery($args: ModelArgs!) {\n  models(args: $args) {\n    nodes {\n      id\n      name\n    }\n    totalNodes\n    pageInfo {\n      hasPreviousPage\n      hasNextPage\n      startCursor\n      endCursor\n    }\n  }\n}" ;
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
    type UUID = crate::graphql::UUID;
    #[derive(Serialize)]
    pub struct ModelArgs {
        pub after: Option<Cursor>,
        pub before: Option<Cursor>,
        pub first: Option<Int>,
        pub last: Option<Int>,
        #[serde(rename = "where")]
        pub where_: Option<ModelWhereInput>,
    }
    #[derive(Serialize)]
    pub struct ModelWhereInput {
        #[serde(rename = "providerId")]
        pub provider_id: Option<ModelWhereProviderIdInput>,
    }
    #[derive(Serialize)]
    pub struct ModelWhereProviderIdInput {
        pub equals: Option<UUID>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub args: ModelArgs,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub models: PaginateModelsQueryModels,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct PaginateModelsQueryModels {
        pub nodes: Vec<PaginateModelsQueryModelsNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: PaginateModelsQueryModelsPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct PaginateModelsQueryModelsNodes {
        pub id: UUID,
        pub name: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct PaginateModelsQueryModelsPageInfo {
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
impl graphql_client::GraphQLQuery for PaginateModelsQuery {
    type Variables = paginate_models_query::Variables;
    type ResponseData = paginate_models_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: paginate_models_query::QUERY,
            operation_name: paginate_models_query::OPERATION_NAME,
        }
    }
}
