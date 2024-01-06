#![allow(clippy::all, warnings)]
pub struct CreateThreadMutation;
pub mod create_thread_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateThreadMutation";
    pub const QUERY : & str = "mutation CreateThreadMutation($input: ThreadCreateInput!) {\n  createThread(input: $input) {\n    id\n    name\n    slug\n    ownerId\n    createdAt\n    updatedAt\n    version {\n      id\n      semver\n      version\n      releaseNote\n      description\n      document\n      status\n      threadId\n      ownerId\n      createdAt\n      updatedAt\n      parameters {\n        id\n        name\n        temperature\n        maxTokens\n        stopSequences\n        topP\n        frequencyPenalty\n        presencePenalty\n        extra\n        modelId\n        threadVersionId\n        createdAt\n        updatedAt\n      }\n      messages {\n        id\n        threadVersionId\n        ownerId\n        raw\n        content\n        role\n        createdAt\n        updatedAt\n      }\n    }\n    owner {\n      id\n      createdAt\n      email\n      role\n      updatedAt\n      username\n    }\n  }\n}" ;
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub enum UserRole {
        ADMIN,
        USER,
        Other(String),
    }
    impl ::serde::Serialize for UserRole {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                UserRole::ADMIN => "ADMIN",
                UserRole::USER => "USER",
                UserRole::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserRole {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "ADMIN" => Ok(UserRole::ADMIN),
                "USER" => Ok(UserRole::USER),
                _ => Ok(UserRole::Other(s)),
            }
        }
    }
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
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "createThread")]
        pub create_thread: CreateThreadMutationCreateThread,
    }
    #[derive(Deserialize, Debug)]
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
        pub version: Option<CreateThreadMutationCreateThreadVersion>,
        pub owner: Option<CreateThreadMutationCreateThreadOwner>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CreateThreadMutationCreateThreadVersion {
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
        pub parameters: Vec<CreateThreadMutationCreateThreadVersionParameters>,
        pub messages: Vec<CreateThreadMutationCreateThreadVersionMessages>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CreateThreadMutationCreateThreadVersionParameters {
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
    #[derive(Deserialize, Debug)]
    pub struct CreateThreadMutationCreateThreadVersionMessages {
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
    #[derive(Deserialize, Debug)]
    pub struct CreateThreadMutationCreateThreadOwner {
        pub id: UUID,
        #[serde(rename = "createdAt")]
        pub created_at: NaiveDateTime,
        pub email: String,
        pub role: UserRole,
        #[serde(rename = "updatedAt")]
        pub updated_at: NaiveDateTime,
        pub username: String,
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
