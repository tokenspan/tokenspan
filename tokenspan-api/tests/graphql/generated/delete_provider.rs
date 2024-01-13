#![allow(clippy::all, warnings)]
pub struct DeleteProviderMutation;
pub mod delete_provider_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteProviderMutation";
    pub const QUERY : & str = "mutation DeleteProviderMutation($id: UUID!) {\n  deleteProvider(id: $id) {\n    id\n    name\n    slug\n    createdAt\n    updatedAt\n  }\n}" ;
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
        pub id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "deleteProvider")]
        pub delete_provider: DeleteProviderMutationDeleteProvider,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct DeleteProviderMutationDeleteProvider {
        pub id: UUID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
}
impl graphql_client::GraphQLQuery for DeleteProviderMutation {
    type Variables = delete_provider_mutation::Variables;
    type ResponseData = delete_provider_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_provider_mutation::QUERY,
            operation_name: delete_provider_mutation::OPERATION_NAME,
        }
    }
}
