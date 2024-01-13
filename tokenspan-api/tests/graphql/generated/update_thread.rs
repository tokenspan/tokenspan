#![allow(clippy::all, warnings)]
pub struct UpdateThreadMutation;
pub mod update_thread_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateThreadMutation";
    pub const QUERY : & str = "mutation UpdateThreadMutation($updateThreadId: UUID!, $input: ThreadUpdateInput!) {\n  updateThread(id: $updateThreadId, input: $input) {\n    id\n    name\n  }\n}" ;
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
    pub struct ThreadUpdateInput {
        pub name: Option<String>,
        pub slug: Option<String>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "updateThreadId")]
        pub update_thread_id: UUID,
        pub input: ThreadUpdateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "updateThread")]
        pub update_thread: UpdateThreadMutationUpdateThread,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct UpdateThreadMutationUpdateThread {
        pub id: UUID,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UpdateThreadMutation {
    type Variables = update_thread_mutation::Variables;
    type ResponseData = update_thread_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_thread_mutation::QUERY,
            operation_name: update_thread_mutation::OPERATION_NAME,
        }
    }
}
