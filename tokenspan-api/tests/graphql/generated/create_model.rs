#![allow(clippy::all, warnings)]
pub struct CreateModelMutation;
pub mod create_model_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateModelMutation";
    pub const QUERY : & str = "mutation CreateModelMutation($input: ModelCreateInput!) {\n  createModel(input: $input) {\n    id\n    name\n    description\n    slug\n    context\n    inputPricing {\n      price\n      tokens\n      currency\n    }\n    outputPricing {\n      price\n      tokens\n      currency\n    }\n    trainingAt\n    providerId\n    createdAt\n    updatedAt\n  }\n}" ;
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
    pub struct ModelCreateInput {
        pub context: Int,
        pub description: String,
        #[serde(rename = "inputPricing")]
        pub input_pricing: PricingInput,
        pub name: String,
        #[serde(rename = "outputPricing")]
        pub output_pricing: PricingInput,
        #[serde(rename = "providerId")]
        pub provider_id: UUID,
        pub slug: String,
        #[serde(rename = "trainingAt")]
        pub training_at: NaiveDateTime,
    }
    #[derive(Serialize)]
    pub struct PricingInput {
        pub currency: String,
        pub price: Float,
        pub tokens: Int,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: ModelCreateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "createModel")]
        pub create_model: CreateModelMutationCreateModel,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CreateModelMutationCreateModel {
        pub id: UUID,
        pub name: String,
        pub description: String,
        pub slug: String,
        pub context: Int,
        #[serde(rename = "inputPricing")]
        pub input_pricing: CreateModelMutationCreateModelInputPricing,
        #[serde(rename = "outputPricing")]
        pub output_pricing: CreateModelMutationCreateModelOutputPricing,
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
    pub struct CreateModelMutationCreateModelInputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CreateModelMutationCreateModelOutputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
}
impl graphql_client::GraphQLQuery for CreateModelMutation {
    type Variables = create_model_mutation::Variables;
    type ResponseData = create_model_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_model_mutation::QUERY,
            operation_name: create_model_mutation::OPERATION_NAME,
        }
    }
}
