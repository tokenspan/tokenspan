use anyhow::Result;
use axum::http::StatusCode;
use axum_test::TestServer;
use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};

use tokenspan_api::app::make_app;
use tokenspan_api::configs;

#[tokio::test]
async fn test_sign_in() -> Result<()> {
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

    // Sign up
    let resp = server.get("api/v1/health").await;
    let status = resp.status_code();

    assert_eq!(status, StatusCode::OK);

    Ok(())
}
