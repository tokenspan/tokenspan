#![allow(clippy::all, warnings)]
pub struct DeleteFunctionMutation;
pub mod delete_function_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteFunctionMutation";
    pub const QUERY : & str = "mutation DeleteFunctionMutation($deleteFunctionId: UUID!) {\n  deleteFunction(id: $deleteFunctionId) {\n    id\n    ownerId\n    name\n    description\n    parameters\n    response\n    createdAt\n    updatedAt\n  }\n}" ;
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
    type JSON = crate::graphql::JSON;
    type NaiveDateTime = crate::graphql::NaiveDateTime;
    type UUID = crate::graphql::UUID;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "deleteFunctionId")]
        pub delete_function_id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "deleteFunction")]
        pub delete_function: DeleteFunctionMutationDeleteFunction,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct DeleteFunctionMutationDeleteFunction {
        pub id: UUID,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        pub name: String,
        pub description: String,
        pub parameters: JSON,
        pub response: Option<JSON>,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
}
impl graphql_client::GraphQLQuery for DeleteFunctionMutation {
    type Variables = delete_function_mutation::Variables;
    type ResponseData = delete_function_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_function_mutation::QUERY,
            operation_name: delete_function_mutation::OPERATION_NAME,
        }
    }
}
