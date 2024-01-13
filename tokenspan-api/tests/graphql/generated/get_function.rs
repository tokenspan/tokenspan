#![allow(clippy::all, warnings)]
pub struct GetFunctionQuery;
pub mod get_function_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetFunctionQuery";
    pub const QUERY : & str = "query GetFunctionQuery($functionId: UUID!) {\n  function(id: $functionId) {\n    id\n    ownerId\n    name\n    description\n    parameters\n    response\n    createdAt\n    updatedAt\n  }\n}" ;
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
        #[serde(rename = "functionId")]
        pub function_id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub function: Option<GetFunctionQueryFunction>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetFunctionQueryFunction {
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
impl graphql_client::GraphQLQuery for GetFunctionQuery {
    type Variables = get_function_query::Variables;
    type ResponseData = get_function_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_function_query::QUERY,
            operation_name: get_function_query::OPERATION_NAME,
        }
    }
}
