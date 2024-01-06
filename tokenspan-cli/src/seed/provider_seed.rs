use async_trait::async_trait;
use serde::Deserialize;
use tokio_stream::StreamExt;

use tokenspan_api::api::dto::ProviderCreateInput;
use tokenspan_api::configs::AppConfig;
use tokenspan_api::state::AppState;

use crate::seed::Seed;

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderRef {
    pub slug: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Provider {
    pub name: String,
    pub slug: String,
}

pub struct ProviderSeed {
    pub data: Vec<Provider>,
    pub config: AppConfig,
    pub state: AppState,
}

impl ProviderSeed {
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
        data: Vec<Provider>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            data,
            config,
            state,
        })
    }
}

#[async_trait]
impl Seed for ProviderSeed {
    async fn save(&self) -> anyhow::Result<()> {
        let provider_service = self.state.provider_service.clone();
        let mut stream = tokio_stream::iter(self.data.clone());
        while let Some(provider) = stream.next().await {
            let result = provider_service.find_by_slug(provider.slug.clone()).await?;
            if let Some(provider) = result {
                println!("Provider: {} already existed", provider.name);
                continue;
            }

            let provider = provider_service
                .create(ProviderCreateInput {
                    name: provider.name,
                    slug: provider.slug,
                })
                .await?;
            println!("Provider: {} created", provider.name)
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/providers"
    }
}
