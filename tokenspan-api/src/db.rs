use crate::configs;
use futures_util::TryFutureExt;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::str::FromStr;
use std::time::Duration;
use tracing::log;

pub async fn connect_db(config: &configs::DatabaseConfig) -> anyhow::Result<DatabaseConnection> {
    let sqlx_logging_level = match config.sqlx_logging_level {
        Some(ref level) => log::LevelFilter::from_str(level).map_err(|e| anyhow::anyhow!(e))?,
        None => log::LevelFilter::Info,
    };

    let mut opt = ConnectOptions::new(&config.url);
    opt.max_connections(config.max_connections.unwrap_or(10))
        .min_connections(config.min_connections.unwrap_or(5))
        .connect_timeout(Duration::from_secs(config.connect_timeout.unwrap_or(8)))
        .acquire_timeout(Duration::from_secs(config.acquire_timeout.unwrap_or(8)))
        .idle_timeout(Duration::from_secs(config.idle_timeout.unwrap_or(8)))
        .max_lifetime(Duration::from_secs(config.max_lifetime.unwrap_or(8)))
        .sqlx_logging(config.sqlx_logging.unwrap_or(true))
        .sqlx_logging_level(sqlx_logging_level);

    let db = Database::connect(opt)
        .map_err(|e| anyhow::anyhow!(e))
        .await?;
    // migration::Migrator::up(&db, None)
    //     .await
    //     .map_err(|e| anyhow::anyhow!(e))?;

    Ok(db)
}
