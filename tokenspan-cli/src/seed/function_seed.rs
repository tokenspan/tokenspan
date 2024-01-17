use async_trait::async_trait;
use dojo_orm::predicates::equals;
use dojo_orm::Database;
use tracing::{info, warn};

use tokenspan_api::domains::models::Function;

use crate::seed::Seed;

pub struct FunctionSeed<'a> {
    pub db: &'a Database,
}

impl<'a> FunctionSeed<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a> Seed for FunctionSeed<'a> {
    async fn save(&self) -> anyhow::Result<()> {
        let items = Self::load::<Function>().await?;

        for item in items {
            let function = self
                .db
                .bind::<Function>()
                .where_by(equals("id", &item.id))
                .first()
                .await?;
            if function.is_some() {
                warn!("Model {} already exists", item.id);
                continue;
            }

            self.db.insert(&item).await?;
            info!("Model {} created", item.id);
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/functions"
    }
}
