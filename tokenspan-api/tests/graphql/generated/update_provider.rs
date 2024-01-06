#![allow(clippy::all, warnings)]
pub struct UpdateProviderMutation;
pub mod update_provider_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateProviderMutation";
    pub const QUERY : & str = "mutation UpdateProviderMutation($id: UUID!, $input: ProviderUpdateInput!) {\n  updateProvider(id: $id, input: $input) {\n    id\n    name\n    slug\n    createdAt\n    updatedAt\n  }\n}" ;
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
    pub struct ProviderUpdateInput {
        pub name: Option<String>,
        pub slug: Option<String>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: UUID,
        pub input: ProviderUpdateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "updateProvider")]
        pub update_provider: Option<UpdateProviderMutationUpdateProvider>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct UpdateProviderMutationUpdateProvider {
        pub id: UUID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
}
impl graphql_client::GraphQLQuery for UpdateProviderMutation {
    type Variables = update_provider_mutation::Variables;
    type ResponseData = update_provider_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_provider_mutation::QUERY,
            operation_name: update_provider_mutation::OPERATION_NAME,
        }
    }
}
