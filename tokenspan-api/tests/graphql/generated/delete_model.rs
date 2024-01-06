#![allow(clippy::all, warnings)]
pub struct DeleteModelMutation;
pub mod delete_model_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteModelMutation";
    pub const QUERY : & str = "mutation DeleteModelMutation($deleteModelId: UUID!) {\n  deleteModel(id: $deleteModelId) {\n    id\n    name\n    description\n    slug\n    context\n    inputPricing {\n      price\n      tokens\n      currency\n    }\n    outputPricing {\n      price\n      tokens\n      currency\n    }\n    trainingAt\n    providerId\n    createdAt\n    updatedAt\n  }\n}" ;
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
        #[serde(rename = "deleteModelId")]
        pub delete_model_id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "deleteModel")]
        pub delete_model: Option<DeleteModelMutationDeleteModel>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct DeleteModelMutationDeleteModel {
        pub id: UUID,
        pub name: String,
        pub description: String,
        pub slug: String,
        pub context: Int,
        #[serde(rename = "inputPricing")]
        pub input_pricing: DeleteModelMutationDeleteModelInputPricing,
        #[serde(rename = "outputPricing")]
        pub output_pricing: DeleteModelMutationDeleteModelOutputPricing,
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
    pub struct DeleteModelMutationDeleteModelInputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct DeleteModelMutationDeleteModelOutputPricing {
        pub price: Float,
        pub tokens: Int,
        pub currency: String,
    }
}
impl graphql_client::GraphQLQuery for DeleteModelMutation {
    type Variables = delete_model_mutation::Variables;
    type ResponseData = delete_model_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_model_mutation::QUERY,
            operation_name: delete_model_mutation::OPERATION_NAME,
        }
    }
}
