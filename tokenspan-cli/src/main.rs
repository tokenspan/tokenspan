use anyhow::Result;
use tokenspan_cli::seed::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = tokenspan_api::configs::AppConfig::new()?;
    let state = tokenspan_api::state::AppState::new(&config).await?;

    let user_seed = UserSeed::new(config.clone(), state.clone()).await?;
    user_seed.save().await?;

    let provider_seed = ProviderSeed::new(config.clone(), state.clone()).await?;
    provider_seed.save().await?;

    let model_seed = ModelSeed::new(config.clone(), state.clone()).await?;
    model_seed.save().await?;

    let thread_seed = ThreadSeed::new(config.clone(), state.clone()).await?;
    thread_seed.save().await?;

    Ok(())
}
