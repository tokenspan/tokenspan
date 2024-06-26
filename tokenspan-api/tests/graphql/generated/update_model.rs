#![allow(clippy::all, warnings)]
pub struct UpdateModelMutation;
pub mod update_model_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateModelMutation";
    pub const QUERY : & str = "mutation UpdateModelMutation($updateModelId: UUID!, $input: ModelUpdateInput!) {\n  updateModel(id: $updateModelId, input: $input) {\n    id\n    name\n    description\n    slug\n    context\n    inputPricing {\n      price\n      tokens\n      currency\n    }\n    outputPricing {\n      price\n      tokens\n      currency\n    }\n    trainingAt\n    providerId\n    createdAt\n    updatedAt\n  }\n}" ;
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
    pub struct ModelUpdateInput {
        pub context: Option<Int>,
        pub description: Option<String>,
        #[serde(rename = "inputPricing")]
        pub input_pricing: Option<PricingInput>,
        pub name: Option<String>,
        #[serde(rename = "outputPricing")]
        pub output_pricing: Option<PricingInput>,
        pub slug: Option<String>,
        #[serde(rename = "trainingAt")]
        pub training_at: Option<NaiveDateTime>,
    }
    #[derive(Serialize)]
    pub struct PricingInput {
        pub currency: String,
        pub price: Float,
        pub tokens: Int,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "updateModelId")]
        pub update_model_id: UUID,
        pub input: ModelUpdateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "updateModel")]
        pub update_model: UpdateModelMutationUpdateModel,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct UpdateModelMutationUpdateModel {
        pub id: UUID,
        pub name: String,
        pub description: String,
        pub slug: String,
        pub context: Int,
        #[serde(rename = "inputPricing")]
        pub input_pricing: UpdateModelMutationUpdateModelInputPricing,
        #[serde(rename = "outputPricing")]
        pub output_pricing: UpdateModelMutationUpdateModelOutputPricing,
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
    pub struct UpdateModelMutationUpdateModelInputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct UpdateModelMutationUpdateModelOutputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
}
impl graphql_client::GraphQLQuery for UpdateModelMutation {
    type Variables = update_model_mutation::Variables;
    type ResponseData = update_model_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_model_mutation::QUERY,
            operation_name: update_model_mutation::OPERATION_NAME,
        }
    }
}
