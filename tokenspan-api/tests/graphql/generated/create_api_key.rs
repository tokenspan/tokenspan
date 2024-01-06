#![allow(clippy::all, warnings)]
pub struct CreateApiKeyMutation;
pub mod create_api_key_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateApiKeyMutation";
    pub const QUERY : & str = "mutation CreateApiKeyMutation($input: ApiKeyCreateInput!) {\n  createApiKey(input: $input) {\n    id\n    name\n    ownerId\n    providerId\n    createdAt\n    updatedAt\n    provider {\n      id\n      name\n      slug\n    }\n    owner {\n      id\n      username\n    }\n  }\n}" ;
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
    pub struct ApiKeyCreateInput {
        pub key: String,
        pub name: String,
        #[serde(rename = "providerId")]
        pub provider_id: UUID,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: ApiKeyCreateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "createApiKey")]
        pub create_api_key: CreateApiKeyMutationCreateApiKey,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CreateApiKeyMutationCreateApiKey {
        pub id: UUID,
        pub name: String,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        #[serde(rename = "providerId")]
        pub provider_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
        pub provider: Option<CreateApiKeyMutationCreateApiKeyProvider>,
        pub owner: Option<CreateApiKeyMutationCreateApiKeyOwner>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CreateApiKeyMutationCreateApiKeyProvider {
        pub id: UUID,
        pub name: String,
        pub slug: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CreateApiKeyMutationCreateApiKeyOwner {
        pub id: UUID,
        pub username: String,
    }
}
impl graphql_client::GraphQLQuery for CreateApiKeyMutation {
    type Variables = create_api_key_mutation::Variables;
    type ResponseData = create_api_key_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_api_key_mutation::QUERY,
            operation_name: create_api_key_mutation::OPERATION_NAME,
        }
    }
}
