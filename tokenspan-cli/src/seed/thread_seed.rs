use async_trait::async_trait;
use dojo_orm::predicates::equals;
use dojo_orm::Database;
use tracing::{info, warn};

use tokenspan_api::domains::models::{Message, Parameter, Thread, ThreadVersion};

use crate::seed::Seed;

pub struct ThreadSeed<'a> {
    pub db: &'a Database,
}

impl<'a> ThreadSeed<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
}

impl<'a> ThreadSeed<'a> {
    async fn save_messages(&self, json_data: serde_json::Value) -> anyhow::Result<()> {
        let messages = serde_json::from_value::<Vec<Message>>(json_data)?;
        info!(?messages);

        for message in messages {
            self.save_message(&message).await?;
        }

        Ok(())
    }

    async fn save_message(&self, data: &Message) -> anyhow::Result<()> {
        let message = self
            .db
            .bind::<Message>()
            .where_by(equals("id", &data.id))
            .first()
            .await?;
        if message.is_some() {
            warn!("Message {} already exists", data.id);
            return Ok(());
        }

        self.db.insert::<Message>(data).exec().await?;
        info!("Message {} created", data.id);

        Ok(())
    }

    async fn save_parameters(&self, json_data: serde_json::Value) -> anyhow::Result<()> {
        let parameters = serde_json::from_value::<Vec<Parameter>>(json_data)?;
        info!(?parameters);

        for parameter in parameters {
            self.save_parameter(&parameter).await?;
        }

        Ok(())
    }

    async fn save_parameter(&self, data: &Parameter) -> anyhow::Result<()> {
        let parameter = self
            .db
            .bind::<Parameter>()
            .where_by(equals("id", &data.id))
            .first()
            .await?;
        if parameter.is_some() {
            warn!("Parameter {} already exists", data.id);
            return Ok(());
        }

        self.db.insert::<Parameter>(data).exec().await?;
        info!("Parameter {} created", data.id);

        Ok(())
    }

    async fn save_thread_versions(&self, json_data: serde_json::Value) -> anyhow::Result<()> {
        for json_data in json_data.as_array().cloned().unwrap_or_default() {
            let version = serde_json::from_value::<ThreadVersion>(json_data.clone())?;
            info!(?version);

            self.save_thread_version(&version).await?;
            self.save_parameters(json_data["parameters"].clone())
                .await?;
            self.save_messages(json_data["messages"].clone()).await?;
        }

        Ok(())
    }

    async fn save_thread_version(&self, data: &ThreadVersion) -> anyhow::Result<()> {
        let thread_version = self
            .db
            .bind::<ThreadVersion>()
            .where_by(equals("id", &data.id))
            .first()
            .await?;
        if thread_version.is_some() {
            warn!("ThreadVersion {} already exists", data.id);
            return Ok(());
        }

        self.db.insert::<ThreadVersion>(data).exec().await?;
        info!("ThreadVersion {} created", data.id);

        Ok(())
    }

    async fn save_thread(&self, item: &Thread) -> anyhow::Result<()> {
        let thread = self
            .db
            .bind::<Thread>()
            .where_by(equals("id", &item.id))
            .first()
            .await?;
        if thread.is_some() {
            warn!("Thread {} already exists", item.id);
            return Ok(());
        }

        self.db.insert(item).exec().await?;
        info!("Thread {} created", item.id);

        Ok(())
    }
}

#[async_trait]
impl<'a> Seed for ThreadSeed<'a> {
    async fn save(&self) -> anyhow::Result<()> {
        let items = Self::load::<serde_json::Value>().await?;

        for item in items {
            let thread = serde_json::from_value::<Thread>(item.clone())?;
            self.save_thread(&thread).await?;
            self.save_thread_versions(item["versions"].clone()).await?;
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/threads"
    }
}
