use async_trait::async_trait;
use dojo_orm::predicates::equals;
use dojo_orm::Database;
use tracing::{info, warn};

use tokenspan_api::domains::models::Model;

use crate::seed::Seed;

pub struct ModelSeed<'a> {
    pub db: &'a Database,
}

impl<'a> ModelSeed<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a> Seed for ModelSeed<'a> {
    async fn save(&self) -> anyhow::Result<()> {
        let items = Self::load::<Model>().await?;

        for item in items {
            let model = self
                .db
                .bind::<Model>()
                .where_by(equals("id", &item.id))
                .first()
                .await?;
            if model.is_some() {
                warn!("Model {} already exists", item.id);
                continue;
            }

            self.db.insert(&item).exec().await?;
            info!("Model {} created", item.id);
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/models"
    }
}
