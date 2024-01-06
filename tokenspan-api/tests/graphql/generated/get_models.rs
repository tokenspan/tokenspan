#![allow(clippy::all, warnings)]
pub struct GetModelsQuery;
pub mod get_models_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetModelsQuery";
    pub const QUERY : & str = "query GetModelsQuery {\n  models {\n    nodes {\n      id\n      name\n      description\n      slug\n      context\n      inputPricing {\n        price\n        tokens\n        currency\n      }\n      outputPricing {\n        price\n        tokens\n        currency\n      }\n      trainingAt\n      providerId\n      createdAt\n      updatedAt\n    }\n    totalNodes\n    pageInfo {\n      hasPreviousPage\n      hasNextPage\n      startCursor\n      endCursor\n    }\n  }\n}" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub models: GetModelsQueryModels,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetModelsQueryModels {
        pub nodes: Vec<GetModelsQueryModelsNodes>,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: GetModelsQueryModelsPageInfo,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetModelsQueryModelsNodes {
        pub id: UUID,
        pub name: String,
        pub description: String,
        pub slug: String,
        pub context: Int,
        #[serde(rename = "inputPricing")]
        pub input_pricing: GetModelsQueryModelsNodesInputPricing,
        #[serde(rename = "outputPricing")]
        pub output_pricing: GetModelsQueryModelsNodesOutputPricing,
        #[serde(rename = "trainingAt")]
        pub training_at: NaiveDateTime,
        #[serde(rename = "providerId")]
        pub provider_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetModelsQueryModelsNodesInputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetModelsQueryModelsNodesOutputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetModelsQueryModelsPageInfo {
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
impl graphql_client::GraphQLQuery for GetModelsQuery {
    type Variables = get_models_query::Variables;
    type ResponseData = get_models_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_models_query::QUERY,
            operation_name: get_models_query::OPERATION_NAME,
        }
    }
}
