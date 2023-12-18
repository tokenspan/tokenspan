use config::{Config, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
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
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<u64>,
    pub acquire_timeout: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub sqlx_logging: Option<bool>,
    pub sqlx_logging_level: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub filter: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub secret: String,
    pub iss: String,
    pub aud: String,
    pub token_exp: i64,
    pub refresh_token_exp: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EncryptionConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: AppEnv,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
    pub auth: AuthConfig,
    pub encryption: EncryptionConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").ok();

        let rust_env = std::env::var("APP__ENV").unwrap_or_else(|_| "development".to_string());

        let env_source = Environment::with_prefix("APP")
            .prefix_separator("__")
            .separator("__");

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", rust_env)).required(false))
            .add_source(File::with_name(".env").required(false))
            .add_source(env_source)
            .set_override_option("APP__DATABASE__URL", database_url)?
            .build()?;

        s.try_deserialize()
    }
}
