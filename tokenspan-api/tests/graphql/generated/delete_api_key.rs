#![allow(clippy::all, warnings)]
pub struct DeleteApiKeyMutation;
pub mod delete_api_key_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteApiKeyMutation";
    pub const QUERY : & str = "mutation DeleteApiKeyMutation($deleteApiKeyId: UUID!) {\n  deleteApiKey(id: $deleteApiKeyId) {\n    id\n    name\n  }\n}" ;
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
    pub struct Variables {
        #[serde(rename = "deleteApiKeyId")]
        pub delete_api_key_id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "deleteApiKey")]
        pub delete_api_key: DeleteApiKeyMutationDeleteApiKey,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct DeleteApiKeyMutationDeleteApiKey {
        pub id: UUID,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for DeleteApiKeyMutation {
    type Variables = delete_api_key_mutation::Variables;
    type ResponseData = delete_api_key_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_api_key_mutation::QUERY,
            operation_name: delete_api_key_mutation::OPERATION_NAME,
        }
    }
}
