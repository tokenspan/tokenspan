#![allow(clippy::all, warnings)]
pub struct UpdateFunctionMutation;
pub mod update_function_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateFunctionMutation";
    pub const QUERY : & str = "mutation UpdateFunctionMutation($updateFunctionId: UUID!, $input: FunctionUpdateInput!) {\n  updateFunction(id: $updateFunctionId, input: $input) {\n    id\n    ownerId\n    name\n    description\n    parameters\n    response\n    createdAt\n    updatedAt\n  }\n}" ;
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
    pub struct FunctionUpdateInput {
        pub description: Option<String>,
        pub name: Option<String>,
        pub parameters: Option<JSON>,
        pub response: Option<JSON>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "updateFunctionId")]
        pub update_function_id: UUID,
        pub input: FunctionUpdateInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "updateFunction")]
        pub update_function: UpdateFunctionMutationUpdateFunction,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct UpdateFunctionMutationUpdateFunction {
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
impl graphql_client::GraphQLQuery for UpdateFunctionMutation {
    type Variables = update_function_mutation::Variables;
    type ResponseData = update_function_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_function_mutation::QUERY,
            operation_name: update_function_mutation::OPERATION_NAME,
        }
    }
}
