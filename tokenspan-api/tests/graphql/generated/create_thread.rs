#![allow(clippy::all, warnings)]
pub struct CreateThreadMutation;
pub mod create_thread_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateThreadMutation";
    pub const QUERY : & str = "mutation CreateThreadMutation($input: ThreadCreateInput!) {\n  createThread(input: $input) {\n    id\n    name\n    slug\n    ownerId\n    createdAt\n    updatedAt\n  }\n}" ;
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
    pub struct ThreadCreateInput {
        pub name: String,
        pub slug: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: ThreadCreateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "createThread")]
        pub create_thread: CreateThreadMutationCreateThread,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CreateThreadMutationCreateThread {
        pub id: UUID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
}
impl graphql_client::GraphQLQuery for CreateThreadMutation {
    type Variables = create_thread_mutation::Variables;
    type ResponseData = create_thread_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_thread_mutation::QUERY,
            operation_name: create_thread_mutation::OPERATION_NAME,
        }
    }
}
