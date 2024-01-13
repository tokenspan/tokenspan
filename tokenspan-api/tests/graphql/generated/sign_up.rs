#![allow(clippy::all, warnings)]
pub struct SignUpMutation;
pub mod sign_up_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SignUpMutation";
    pub const QUERY : & str = "mutation SignUpMutation($input: SignUpInput!) {\n  signUp(input: $input) {\n    refreshToken\n    token\n    user {\n      id\n      email\n      username\n      role\n    }\n  }\n}\n" ;
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
    #[derive(Debug, PartialEq)]
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
    pub struct SignUpInput {
        pub email: String,
        pub password: String,
        pub username: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: SignUpInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "signUp")]
        pub sign_up: SignUpMutationSignUp,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct SignUpMutationSignUp {
        #[serde(rename = "refreshToken")]
        pub refresh_token: String,
        pub token: String,
        pub user: SignUpMutationSignUpUser,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct SignUpMutationSignUpUser {
        pub id: UUID,
        pub email: String,
        pub username: String,
        pub role: UserRole,
    }
}
impl graphql_client::GraphQLQuery for SignUpMutation {
    type Variables = sign_up_mutation::Variables;
    type ResponseData = sign_up_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sign_up_mutation::QUERY,
            operation_name: sign_up_mutation::OPERATION_NAME,
        }
    }
}
