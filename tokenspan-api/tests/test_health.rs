use anyhow::Result;
use axum::http::StatusCode;
use axum_test::TestServer;

use tokenspan_api::state::AppState;

mod common;

#[tokio::test]
async fn test_sign_in() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Sign up
    let resp = server.get("api/v1/health").await;
    let status = resp.status_code();

    assert_eq!(status, StatusCode::OK);

    Ok(())
}
