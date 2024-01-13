#![allow(clippy::all, warnings)]
pub struct CreateProviderMutation;
pub mod create_provider_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateProviderMutation";
    pub const QUERY : & str = "mutation CreateProviderMutation($input: ProviderCreateInput!) {\n  createProvider(input: $input) {\n    id\n    name\n    slug\n    createdAt\n    updatedAt\n  }\n}" ;
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
    pub struct ProviderCreateInput {
        pub name: String,
        pub slug: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: ProviderCreateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "createProvider")]
        pub create_provider: CreateProviderMutationCreateProvider,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CreateProviderMutationCreateProvider {
        pub id: UUID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
}
impl graphql_client::GraphQLQuery for CreateProviderMutation {
    type Variables = create_provider_mutation::Variables;
    type ResponseData = create_provider_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_provider_mutation::QUERY,
            operation_name: create_provider_mutation::OPERATION_NAME,
        }
    }
}
