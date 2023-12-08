use chrono::{DateTime, Utc};
use config::{Config, File};
use dotenv::dotenv;
use serde::Deserialize;
use tokenspan_api::api::dto::PricingInput;
use tokenspan_api::api::types::Role;

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderSeed {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderRef {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PricingSeed {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

impl From<PricingSeed> for PricingInput {
    fn from(value: PricingSeed) -> Self {
        Self {
            price: value.price,
            tokens: value.tokens,
            currency: value.currency,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelSeed {
    pub provider: ProviderRef,
    pub name: String,
    pub description: String,
    pub context: u32,
    pub input_pricing: PricingSeed,
    pub output_pricing: PricingSeed,
    pub training_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserSeed {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SeedConfig {
    pub providers: Vec<ProviderSeed>,
    pub models: Vec<ModelSeed>,
    pub users: Vec<UserSeed>,
}

impl SeedConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        dotenv().ok();

        let s = Config::builder()
            .add_source(File::with_name("tokenspan-migrations/seed/providers"))
            .add_source(File::with_name("tokenspan-migrations/seed/models"))
            .add_source(File::with_name("tokenspan-migrations/seed/users"))
            .build()?;

        s.try_deserialize()
    }
}
