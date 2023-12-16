use crate::seed::prelude::*;
use anyhow::Result;

mod seed;

#[tokio::main]
async fn main() -> Result<()> {
    let config = tokenspan_api::configs::AppConfig::new()?;
    let state = tokenspan_api::state::AppState::new(config.clone()).await?;

    let user_seed = UserSeed::new(config.clone(), state.clone()).await?;
    user_seed.save().await?;

    let provider_seed = ProviderSeed::new(config.clone(), state.clone()).await?;
    provider_seed.save().await?;

    let model_seed = ModelSeed::new(config.clone(), state.clone()).await?;
    model_seed.save().await?;

    let task_seed = TaskSeed::new(config.clone(), state.clone()).await?;
    task_seed.save().await?;

    Ok(())
}
