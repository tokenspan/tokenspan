use async_trait::async_trait;
use dojo_orm::predicates::equals;
use dojo_orm::Database;
use tracing::{info, warn};

use tokenspan_api::domains::models::Provider;

use crate::seed::Seed;

pub struct ProviderSeed<'a> {
    pub db: &'a Database,
}

impl<'a> ProviderSeed<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a> Seed for ProviderSeed<'a> {
    async fn save(&self) -> anyhow::Result<()> {
        let items = Self::load::<Provider>().await?;

        for item in items {
            let provider = self
                .db
                .bind::<Provider>()
                .where_by(equals("id", &item.id))
                .first()
                .await?;
            if provider.is_some() {
                warn!("Provider {} already exists", item.id);
                continue;
            }

            self.db.insert(&item).exec().await?;
            info!("Provider {} created", item.id);
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/providers"
    }
}
