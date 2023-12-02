use config::{Config, Environment, File};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AppEnv {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "production")]
    Production,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub filter: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: AppEnv,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let rust_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        let env_source = Environment::with_prefix("APP").separator("__");

        let s = Config::builder()
            .add_source(File::with_name("tokenspan-api/config/default"))
            .add_source(
                File::with_name(&format!("tokenspan-api/config/{}", rust_env)).required(false),
            )
            .add_source(env_source)
            .build()?;

        info!("Loaded configs: {:?}", s);

        s.try_deserialize()
    }
}
