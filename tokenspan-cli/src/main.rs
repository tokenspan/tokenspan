use crate::seed::prelude::*;
use anyhow::Result;
use tokenspan_api::db::connect_db;

mod seed;

#[tokio::main]
async fn main() -> Result<()> {
    let config = tokenspan_api::configs::AppConfig::new()?;
    let db = connect_db(&config.database).await?;
    let state = tokenspan_api::state::AppState::new(db, &config).await?;

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
