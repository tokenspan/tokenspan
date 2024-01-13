#![allow(clippy::all, warnings)]
pub struct SignInMutation;
pub mod sign_in_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SignInMutation";
    pub const QUERY : & str = "mutation SignInMutation($input: SignInInput!) {\n  signIn(input: $input) {\n    token\n    refreshToken\n    user {\n      id\n      email\n      role\n    }\n  }\n}\n" ;
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
    pub struct SignInInput {
        pub email: String,
        pub password: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub input: SignInInput,
    }
    impl Variables {}
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct ResponseData {
        #[serde(rename = "signIn")]
        pub sign_in: SignInMutationSignIn,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct SignInMutationSignIn {
        pub token: String,
        #[serde(rename = "refreshToken")]
        pub refresh_token: String,
        pub user: SignInMutationSignInUser,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct SignInMutationSignInUser {
        pub id: UUID,
        pub email: String,
        pub role: UserRole,
    }
}
impl graphql_client::GraphQLQuery for SignInMutation {
    type Variables = sign_in_mutation::Variables;
    type ResponseData = sign_in_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sign_in_mutation::QUERY,
            operation_name: sign_in_mutation::OPERATION_NAME,
        }
    }
}
