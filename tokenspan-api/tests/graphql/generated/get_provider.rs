#![allow(clippy::all, warnings)]
pub struct GetProviderQuery;
pub mod get_provider_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetProviderQuery";
    pub const QUERY : & str = "query GetProviderQuery($providerId: UUID!) {\n  provider(id: $providerId) {\n    id\n    name\n    slug\n    createdAt\n    updatedAt\n  }\n}" ;
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
    pub struct Variables {
        #[serde(rename = "providerId")]
        pub provider_id: UUID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        pub provider: Option<GetProviderQueryProvider>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetProviderQueryProvider {
        pub id: UUID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
}
impl graphql_client::GraphQLQuery for GetProviderQuery {
    type Variables = get_provider_query::Variables;
    type ResponseData = get_provider_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_provider_query::QUERY,
            operation_name: get_provider_query::OPERATION_NAME,
        }
    }
}
