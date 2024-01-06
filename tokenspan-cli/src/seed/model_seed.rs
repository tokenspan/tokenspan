use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Deserialize;
use tokio_stream::StreamExt;

use tokenspan_api::api::dto::{ModelCreateInput, PricingInput};
use tokenspan_api::configs::AppConfig;
use tokenspan_api::state::AppState;

use crate::seed::prelude::ProviderRef;
use crate::seed::Seed;

#[derive(Debug, Deserialize, Clone)]
pub struct ModelRef {
    pub slug: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub provider: ProviderRef,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub context: i32,
    pub input_pricing: PricingInput,
    pub output_pricing: PricingInput,
    pub training_at: NaiveDateTime,
}

pub struct ModelSeed {
    pub data: Vec<Model>,
    pub config: AppConfig,
    pub state: AppState,
}

impl ModelSeed {
    pub async fn new(config: AppConfig, state: AppState) -> anyhow::Result<Self> {
        let data = Self::load().await?;
        Ok(Self {
            data,
            config,
            state,
        })
    }

    pub async fn new_with_data(
        config: AppConfig,
        state: AppState,
        data: Vec<Model>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            data,
            config,
            state,
        })
    }
}

#[async_trait]
impl Seed for ModelSeed {
    async fn save(&self) -> anyhow::Result<()> {
        let model_service = self.state.model_service.clone();
        let provider_service = self.state.provider_service.clone();

        let mut stream = tokio_stream::iter(self.data.clone());
        while let Some(model) = stream.next().await {
            let result = model_service.find_by_slug(model.name.clone()).await?;
            if let Some(model) = result {
                println!("Model: {} already existed", model.name);
                continue;
            }

            let provider = provider_service
                .find_by_slug(model.provider.slug.clone())
                .await?
                .ok_or(anyhow::anyhow!("Provider not found"))?;

            let model = model_service
                .create(ModelCreateInput {
                    provider_id: provider.id,
                    name: model.name,
                    description: model.description,
                    slug: model.slug,
                    context: model.context,
                    input_pricing: model.input_pricing,
                    output_pricing: model.output_pricing,
                    training_at: model.training_at,
                })
                .await?;
            println!("Model: {} created", model.name);
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/models"
    }
}
