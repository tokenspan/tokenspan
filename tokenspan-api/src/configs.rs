use config::{Config, Environment, File};
use serde::Deserialize;

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
pub struct AuthConfig {
    pub secret: String,
    pub iss: String,
    pub aud: String,
    pub token_exp: i64,
    pub refresh_token_exp: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: AppEnv,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
    pub auth: AuthConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let rust_env = std::env::var("APP__ENV").unwrap_or_else(|_| "development".to_string());

        let env_source = Environment::with_prefix("APP")
            .prefix_separator("__")
            .separator("__");

        let s = Config::builder()
            .add_source(File::with_name("tokenspan-api/config/default"))
            .add_source(
                File::with_name(&format!("tokenspan-api/config/{}", rust_env)).required(false),
            )
            .add_source(File::with_name("tokenspan-api/.env").required(false))
            .add_source(env_source)
            .build()?;

        s.try_deserialize()
    }
}
