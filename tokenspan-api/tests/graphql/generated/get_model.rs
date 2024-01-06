#![allow(clippy::all, warnings)]
pub struct GetModelQuery;
pub mod get_model_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetModelQuery";
    pub const QUERY : & str = "query GetModelQuery($modelId: UUID!) {\n  model(id: $modelId) {\n    id\n    name\n    description\n    slug\n    context\n    inputPricing {\n      price\n      tokens\n      currency\n    }\n    outputPricing {\n      price\n      tokens\n      currency\n    }\n    trainingAt\n    providerId\n    createdAt\n    updatedAt\n  }\n}" ;
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
        #[serde(rename = "modelId")]
        pub model_id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub model: Option<GetModelQueryModel>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetModelQueryModel {
        pub id: UUID,
        pub name: String,
        pub description: String,
        pub slug: String,
        pub context: Int,
        #[serde(rename = "inputPricing")]
        pub input_pricing: GetModelQueryModelInputPricing,
        #[serde(rename = "outputPricing")]
        pub output_pricing: GetModelQueryModelOutputPricing,
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
    pub struct GetModelQueryModelInputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetModelQueryModelOutputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
}
impl graphql_client::GraphQLQuery for GetModelQuery {
    type Variables = get_model_query::Variables;
    type ResponseData = get_model_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_model_query::QUERY,
            operation_name: get_model_query::OPERATION_NAME,
        }
    }
}
