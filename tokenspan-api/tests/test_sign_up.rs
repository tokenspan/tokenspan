use anyhow::Result;
use axum_test::TestServer;
use graphql_client::{GraphQLQuery, Response};
use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};

use googletest::prelude::*;
use tokenspan_api::app::make_app;
use tokenspan_api::configs;

type UUID = uuid::Uuid;

#[tokio::test]
async fn test_sign_up() -> Result<()> {
    // Setup
    let docker = Cli::default();
    let node = docker.run(Postgres::default());

    let conn_url = &format!(
        "postgres://postgres:postgres@localhost:{}/postgres",
        node.get_host_port_ipv4(5432)
    );

    let mut config = configs::AppConfig::new().expect("Failed to load config");
    config.database.url = conn_url.to_string();

    let app = make_app(config).await?;
    let server = TestServer::new(app)?;

    // GraphQL
    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "../schema.graphql",
        query_path = "tests/graphql/auth/sign-up.graphql",
        response_derives = "Debug"
    )]
    struct SignUpMutation;

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
                    email: eq("linh@gmail.com")
                })
            })
        }))
    );

    Ok(())
}
