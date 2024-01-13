#![allow(clippy::all, warnings)]
pub struct PublishThreadVersionMutation;
pub mod publish_thread_version_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PublishThreadVersionMutation";
    pub const QUERY : & str = "mutation PublishThreadVersionMutation($publishThreadVersionId: UUID!, $input: ThreadVersionPublishInput!) {\n  publishThreadVersion(id: $publishThreadVersionId, input: $input) {\n    id\n  }\n}" ;
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
    pub struct ThreadVersionPublishInput {
        #[serde(rename = "releaseNote")]
        pub release_note: String,
        pub semver: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "publishThreadVersionId")]
        pub publish_thread_version_id: UUID,
        pub input: ThreadVersionPublishInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "publishThreadVersion")]
        pub publish_thread_version: PublishThreadVersionMutationPublishThreadVersion,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct PublishThreadVersionMutationPublishThreadVersion {
        pub id: UUID,
    }
}
impl graphql_client::GraphQLQuery for PublishThreadVersionMutation {
    type Variables = publish_thread_version_mutation::Variables;
    type ResponseData = publish_thread_version_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: publish_thread_version_mutation::QUERY,
            operation_name: publish_thread_version_mutation::OPERATION_NAME,
        }
    }
}
