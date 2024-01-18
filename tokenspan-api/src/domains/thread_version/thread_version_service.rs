use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::prelude::*;
use dojo_orm::Database;
use tracing::info;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::domains::dto::ThreadVersionPublishInput;
use crate::domains::models::{ThreadVersion, ThreadVersionStatus};
use crate::domains::services::{MessageServiceDyn, ParameterServiceDyn};
use crate::domains::thread_version::dto::{
    ThreadVersionArgs, ThreadVersionCreateInput, ThreadVersionUpdateInput,
};

#[async_trait::async_trait]
pub trait ThreadVersionServiceExt {
    async fn paginate(&self, args: ThreadVersionArgs) -> Result<Pagination<ThreadVersion>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<ThreadVersion>>;
    async fn find_by_semver(
        &self,
        thread_id: &Uuid,
        semver: &String,
    ) -> Result<Option<ThreadVersion>>;
    async fn find_latest(&self, thread_id: &Uuid) -> Result<Option<ThreadVersion>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<ThreadVersion>>;
    async fn create(
        &self,
        input: ThreadVersionCreateInput,
        owner_id: Uuid,
    ) -> Result<ThreadVersion>;
    async fn publish(
        &self,
        id: &Uuid,
        input: ThreadVersionPublishInput,
        owner_id: Uuid,
    ) -> Result<ThreadVersion>;
    async fn update_by_id(
        &self,
        id: &Uuid,
        input: ThreadVersionUpdateInput,
    ) -> Result<ThreadVersion>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<ThreadVersion>;
}

pub type ThreadVersionServiceDyn = Arc<dyn ThreadVersionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ThreadVersionService {
    db: Database,
    parameter_service: ParameterServiceDyn,
    message_service: MessageServiceDyn,
}

#[async_trait::async_trait]
impl ThreadVersionServiceExt for ThreadVersionService {
    async fn paginate(&self, args: ThreadVersionArgs) -> Result<Pagination<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_semver(
        &self,
        thread_id: &Uuid,
        semver: &String,
    ) -> Result<Option<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(and(&[
                equals("thread_id", thread_id),
                equals("semver", semver),
            ]))
            .first()
            .await
    }

    async fn find_latest(&self, thread_id: &Uuid) -> Result<Option<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(equals("thread_id", thread_id))
            .order_by(desc("version"))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(in_list("id", &ids))
            .all()
            .await
    }

    async fn create(
        &self,
        input: ThreadVersionCreateInput,
        owner_id: Uuid,
    ) -> Result<ThreadVersion> {
        let input = ThreadVersion {
            id: Uuid::new_v4(),
            version: input.version,
            semver: input.semver,
            status: ThreadVersionStatus::Draft,
            document: None,
            release_note: None,
            description: None,
            thread_id: input.thread_id,
            owner_id,
            published_at: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).exec().await
    }

    async fn publish(
        &self,
        id: &Uuid,
        input: ThreadVersionPublishInput,
        owner_id: Uuid,
    ) -> Result<ThreadVersion> {
        let thread_version = self
            .find_by_id(id)
            .await?
            .ok_or(anyhow::anyhow!("thread version not found"))?;

        if thread_version.status == ThreadVersionStatus::Published {
            return Err(anyhow::anyhow!("thread version already published"));
        }

        let update_input = ThreadVersionUpdateInput {
            release_note: Some(input.release_note),
            status: Some(ThreadVersionStatus::Published),
            ..Default::default()
        };
        let current_thread_version = self.update_by_id(id, update_input).await?;

        let new_version = current_thread_version.version + 1;
        let input = ThreadVersion {
            id: Uuid::new_v4(),
            owner_id,
            description: current_thread_version.description,
            document: current_thread_version.document,
            release_note: current_thread_version.release_note,
            semver: input.semver,
            version: new_version,
            thread_id: current_thread_version.thread_id,
            published_at: None,
            status: ThreadVersionStatus::Draft,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        let new_thread_version = self.db.insert(&input).exec().await?;

        self.parameter_service
            .duplicate_by_thread_version_id(&current_thread_version.id, new_thread_version.id)
            .await?;
        self.message_service
            .duplicate_by_thread_version_id(&current_thread_version.id, new_thread_version.id)
            .await?;

        Ok(new_thread_version)
    }

    async fn update_by_id(
        &self,
        id: &Uuid,
        input: ThreadVersionUpdateInput,
    ) -> Result<ThreadVersion> {
        info!("update thread_version: id: {}, input: {:?}", id, input);
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<ThreadVersion> {
        let thread_version = self
            .find_by_id(id)
            .await?
            .ok_or(anyhow::anyhow!("thread version not found"))?;

        if thread_version.status == ThreadVersionStatus::Draft {
            return Err(anyhow::anyhow!("thread version is draft"));
        }

        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<ThreadVersionService> for ThreadVersionServiceDyn {
    fn from(value: ThreadVersionService) -> Self {
        Arc::new(value) as Self
    }
}
