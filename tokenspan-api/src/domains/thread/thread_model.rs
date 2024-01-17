use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::NaiveDateTime;
use dojo_macros::Model;
use serde::{Deserialize, Serialize};

use crate::api::loaders::UserLoader;
use uuid::Uuid;

use crate::api::models::{ThreadVersion, User};
use crate::api::services::ThreadVersionServiceDyn;
use crate::error::AppError;

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize, Model)]
#[graphql(complex)]
#[dojo(name = "threads", sort_keys = ["created_at", "id"])]
pub struct Thread {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub owner_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Thread {
    pub async fn version<'a>(
        &self,
        ctx: &Context<'a>,
        semver: Option<String>,
    ) -> Result<Option<ThreadVersion>> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread_version = if let Some(semver) = semver {
            thread_version_service
                .find_by_semver(&self.id, &semver)
                .await?
        } else {
            thread_version_service.find_latest(&self.id).await?
        };

        Ok(thread_version)
    }

    pub async fn owner<'a>(&self, ctx: &Context<'a>) -> Result<Option<User>> {
        let user_loader = ctx
            .data::<DataLoader<UserLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let user = user_loader.load_one(self.owner_id).await?;

        Ok(user)
    }
}
