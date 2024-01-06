#![allow(clippy::all, warnings)]
pub struct UpdateApiKeyMutation;
pub mod update_api_key_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateApiKeyMutation";
    pub const QUERY : & str = "mutation UpdateApiKeyMutation($updateApiKeyId: UUID!, $input: ApiKeyUpdateInput!) {\n  updateApiKey(id: $updateApiKeyId, input: $input) {\n    id\n    name\n  }\n}" ;
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
    type UUID = crate::graphql::UUID;
    #[derive(Serialize)]
    pub struct ApiKeyUpdateInput {
        pub name: Option<String>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "updateApiKeyId")]
        pub update_api_key_id: UUID,
        pub input: ApiKeyUpdateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "updateApiKey")]
        pub update_api_key: Option<UpdateApiKeyMutationUpdateApiKey>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct UpdateApiKeyMutationUpdateApiKey {
        pub id: UUID,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UpdateApiKeyMutation {
    type Variables = update_api_key_mutation::Variables;
    type ResponseData = update_api_key_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_api_key_mutation::QUERY,
            operation_name: update_api_key_mutation::OPERATION_NAME,
        }
    }
}
