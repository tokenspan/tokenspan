use anyhow::Result;
use dojo_orm::Database;
use std::ops::DerefMut;
use tokenspan_cli::seed::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("../tokenspan-api/migrations");
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("tokenspan_cli=info")),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
    let db = Database::new("postgres://postgres:123456@localhost:5432/tokenspan").await?;
    let mut conn = db.get().await?;
    let client = conn.deref_mut();
    embedded::migrations::runner().run_async(client).await?;

    let user_seed = UserSeed::new(&db);
    user_seed.save().await?;

    let provider_seed = ProviderSeed::new(&db);
    provider_seed.save().await?;

    let model_seed = ModelSeed::new(&db);
    model_seed.save().await?;

    let function_seed = FunctionSeed::new(&db);
    function_seed.save().await?;

    let thread_seed = ThreadSeed::new(&db);
    thread_seed.save().await?;

    let function_seed = FunctionSeed::new(&db);
    function_seed.save().await?;

    Ok(())
}
