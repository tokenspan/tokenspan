#![allow(clippy::all, warnings)]
pub struct DeleteFunction;
pub mod delete_function {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteFunction";
    pub const QUERY : & str = "mutation DeleteFunction($deleteFunctionId: UUID!) {\n  deleteFunction(id: $deleteFunctionId) {\n    id\n    ownerId\n    name\n    description\n    parameters\n    response\n    createdAt\n    updatedAt\n  }\n}" ;
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
        pub delete_function: DeleteFunctionDeleteFunction,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct DeleteFunctionDeleteFunction {
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
impl graphql_client::GraphQLQuery for DeleteFunction {
    type Variables = delete_function::Variables;
    type ResponseData = delete_function::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_function::QUERY,
            operation_name: delete_function::OPERATION_NAME,
        }
    }
}
