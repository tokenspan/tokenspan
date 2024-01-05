use anyhow::Result;
use axum_test::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use graphql_client::{GraphQLQuery, Response};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::clients::Cli;

use tokenspan_api::api::models::UserRole;
use tokenspan_api::app::make_app_with_state;
use tokenspan_api::configs;

type UUID = uuid::Uuid;
type NaiveDateTime = chrono::NaiveDateTime;

#[tokio::test]
async fn test_create_provider() -> Result<()> {
    // Setup
    let docker = Cli::default();
    let node = docker.run(Postgres::default());

    let conn_url = &format!(
        "postgres://postgres:postgres@localhost:{}/postgres",
        node.get_host_port_ipv4(5432)
    );

    let mut config = configs::AppConfig::new().expect("Failed to load config");
    config.database.url = conn_url.to_string();

    let state = tokenspan_api::state::AppState::new(&config).await?;
    let app = make_app_with_state(config, state.clone()).await?;
    let server = TestServer::new(app)?;

    // create new user
    let auth_payload = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    // GraphQL
    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "../schema.graphql",
        query_path = "tests/graphql/provider/create-provider.graphql",
        response_derives = "Debug"
    )]
    struct CreateProviderMutation;

    // Create provider
    let variables = create_provider_mutation::Variables {
        input: create_provider_mutation::ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
        },
    };
    let req_body = CreateProviderMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_payload.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<create_provider_mutation::ResponseData>>();
    println!("{:?}", resp);

    Ok(())
}
