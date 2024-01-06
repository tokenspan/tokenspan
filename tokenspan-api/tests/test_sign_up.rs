use anyhow::Result;
use axum_test::TestServer;
use googletest::prelude::*;
use graphql_client::{GraphQLQuery, Response};

use tokenspan_api::state::AppState;

use crate::graphql::sign_up_mutation::UserRole;
use crate::graphql::{sign_up_mutation, SignUpMutation};

mod common;
mod graphql;

#[tokio::test]
async fn test_sign_up() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Sign up
    let variables = sign_up_mutation::Variables {
        input: sign_up_mutation::SignUpInput {
            username: "linh".to_string(),
            email: "linh@gmail.com".to_string(),
            password: "123".to_string(),
        },
    };
    let req_body = SignUpMutation::build_query(variables);
    let resp = server.post("graphql").json(&req_body).await;
    let resp = resp.json::<Response<sign_up_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(sign_up_mutation::ResponseData {
            sign_up: pat!(sign_up_mutation::SignUpMutationSignUp {
                token: anything(),
                refresh_token: anything(),
                user: pat!(sign_up_mutation::SignUpMutationSignUpUser {
                    id: anything(),
                    username: eq("linh"),
                    email: eq("linh@gmail.com"),
                    role: eq(UserRole::USER),
                })
            })
        }))
    );

    Ok(())
}
