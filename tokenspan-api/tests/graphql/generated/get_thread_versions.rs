#![allow(clippy::all, warnings)]
pub struct GetThreadVersionsQuery;
pub mod get_thread_versions_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetThreadVersionsQuery";
    pub const QUERY : & str = "query GetThreadVersionsQuery {\n  threadVersions {\n    nodes {\n      id\n      semver\n      version\n      releaseNote\n      description\n      document\n      status\n      threadId\n      ownerId\n      createdAt\n      updatedAt\n      parameters {\n        id\n        name\n        temperature\n        maxTokens\n        stopSequences\n        topP\n        frequencyPenalty\n        presencePenalty\n        extra\n        modelId\n        threadVersionId\n        createdAt\n        updatedAt\n      }\n      messages {\n        id\n        threadVersionId\n        ownerId\n        raw\n        content\n        role\n        createdAt\n        updatedAt\n      }\n    }\n    pageInfo {\n      hasPreviousPage\n      hasNextPage\n      startCursor\n      endCursor\n    }\n    totalNodes\n  }\n}" ;
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
    #[derive(Debug, PartialEq)]
    pub enum ThreadVersionStatus {
        DRAFT,
        PUBLISHED,
        Other(String),
    }
    impl ::serde::Serialize for ThreadVersionStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                ThreadVersionStatus::DRAFT => "DRAFT",
                ThreadVersionStatus::PUBLISHED => "PUBLISHED",
                ThreadVersionStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThreadVersionStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "DRAFT" => Ok(ThreadVersionStatus::DRAFT),
                "PUBLISHED" => Ok(ThreadVersionStatus::PUBLISHED),
                _ => Ok(ThreadVersionStatus::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "threadVersions")]
        pub thread_versions: GetThreadVersionsQueryThreadVersions,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersions {
        pub nodes: Vec<GetThreadVersionsQueryThreadVersionsNodes>,
        #[serde(rename = "pageInfo")]
        pub page_info: GetThreadVersionsQueryThreadVersionsPageInfo,
        #[serde(rename = "totalNodes")]
        pub total_nodes: Int,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersionsNodes {
        pub id: UUID,
        pub semver: String,
        pub version: Int,
        #[serde(rename = "releaseNote")]
        pub release_note: Option<String>,
        pub description: Option<String>,
        pub document: Option<String>,
        pub status: ThreadVersionStatus,
        #[serde(rename = "threadId")]
        pub thread_id: UUID,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
        pub parameters: Vec<GetThreadVersionsQueryThreadVersionsNodesParameters>,
        pub messages: Vec<GetThreadVersionsQueryThreadVersionsNodesMessages>,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersionsNodesParameters {
        pub id: UUID,
        pub name: String,
        pub temperature: Float,
        #[serde(rename = "maxTokens")]
        pub max_tokens: Int,
        #[serde(rename = "stopSequences")]
        pub stop_sequences: Vec<String>,
        #[serde(rename = "topP")]
        pub top_p: Float,
        #[serde(rename = "frequencyPenalty")]
        pub frequency_penalty: Float,
        #[serde(rename = "presencePenalty")]
        pub presence_penalty: Float,
        pub extra: Option<JSON>,
        #[serde(rename = "modelId")]
        pub model_id: UUID,
        #[serde(rename = "threadVersionId")]
        pub thread_version_id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersionsNodesMessages {
        pub id: UUID,
        #[serde(rename = "threadVersionId")]
        pub thread_version_id: UUID,
        #[serde(rename = "ownerId")]
        pub owner_id: UUID,
        pub raw: String,
        pub content: String,
        pub role: String,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct GetThreadVersionsQueryThreadVersionsPageInfo {
        #[serde(rename = "hasPreviousPage")]
        pub has_previous_page: Boolean,
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "startCursor")]
        pub start_cursor: Option<String>,
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for GetThreadVersionsQuery {
    type Variables = get_thread_versions_query::Variables;
    type ResponseData = get_thread_versions_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_thread_versions_query::QUERY,
            operation_name: get_thread_versions_query::OPERATION_NAME,
        }
    }
}
