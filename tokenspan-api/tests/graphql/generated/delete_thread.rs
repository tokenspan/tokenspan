#![allow(clippy::all, warnings)]
pub struct DeleteThreadMutation;
pub mod delete_thread_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteThreadMutation";
    pub const QUERY : & str = "mutation DeleteThreadMutation($deleteThreadId: UUID!) {\n  deleteThread(id: $deleteThreadId) {\n    id\n    name\n  }\n}" ;
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
        #[serde(rename = "deleteThreadId")]
        pub delete_thread_id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "deleteThread")]
        pub delete_thread: Option<DeleteThreadMutationDeleteThread>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct DeleteThreadMutationDeleteThread {
        pub id: UUID,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for DeleteThreadMutation {
    type Variables = delete_thread_mutation::Variables;
    type ResponseData = delete_thread_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_thread_mutation::QUERY,
            operation_name: delete_thread_mutation::OPERATION_NAME,
        }
    }
}
