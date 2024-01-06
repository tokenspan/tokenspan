use std::env;

use anyhow::Result;
use axum_test::TestServer;
use googletest::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};

use tokenspan_api::app::make_app_with_state;
use tokenspan_api::configs;
use tokenspan_api::state::AppState;

use crate::graphql::sign_in_mutation::UserRole;
use crate::graphql::{sign_in_mutation, SignInMutation};

mod common;
mod graphql;

#[tokio::test]
async fn test_sign_in() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Sign up
    state
        .auth_service
        .sign_up(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
        )
        .await?;

    // Sign in
    let variables = sign_in_mutation::Variables {
        input: sign_in_mutation::SignInInput {
            email: "linh@gmail.com".to_string(),
            password: "123".to_string(),
        },
    };
    let req_body = SignInMutation::build_query(variables);
    let resp = server.post("graphql").json(&req_body).await;
    let resp = resp.json::<Response<sign_in_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(sign_in_mutation::ResponseData {
            sign_in: pat!(sign_in_mutation::SignInMutationSignIn {
                token: anything(),
                refresh_token: anything(),
                user: pat!(sign_in_mutation::SignInMutationSignInUser {
                    id: anything(),
                    email: eq("linh@gmail.com"),
                    role: eq(UserRole::USER),
                })
            })
        }))
    );

    Ok(())
}
