use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

use tokenspan_api::app::make_app_with_state;
use tokenspan_api::{configs, state};

#[tokio::main]
async fn main() -> Result<()> {
    let config = configs::AppConfig::new().expect("Failed to load config");
    let state = state::AppState::new(&config).await?;
    let app = make_app_with_state(config, state).await?;

    info!("Sandbox: http://localhost:8080/graphql");

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
