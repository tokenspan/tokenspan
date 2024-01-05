use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::ops::{and, desc, eq, in_list};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
use tracing::info;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::models::{ThreadVersion, ThreadVersionStatus};
use crate::api::thread_version::dto::{
    ThreadVersionArgs, ThreadVersionCreateInput, ThreadVersionUpdateInput,
};

#[async_trait::async_trait]
pub trait ThreadVersionServiceExt {
    async fn paginate(&self, args: ThreadVersionArgs) -> Result<Pagination<Cursor, ThreadVersion>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ThreadVersion>>;
    async fn find_by_semver(
        &self,
        thread_id: Uuid,
        semver: String,
    ) -> Result<Option<ThreadVersion>>;
    async fn find_latest(&self, thread_id: Uuid) -> Result<Option<ThreadVersion>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ThreadVersion>>;
    async fn create(
        &self,
        input: ThreadVersionCreateInput,
        owner_id: Uuid,
    ) -> Result<ThreadVersion>;
    async fn update_by_id(
        &self,
        id: Uuid,
        input: ThreadVersionUpdateInput,
    ) -> Result<Option<ThreadVersion>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<ThreadVersion>>;
    async fn release(&self, id: Uuid) -> Result<ThreadVersion>;
}

pub type ThreadVersionServiceDyn = Arc<dyn ThreadVersionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ThreadVersionService {
    db: Database,
}

#[async_trait::async_trait]
impl ThreadVersionServiceExt for ThreadVersionService {
    async fn paginate(&self, args: ThreadVersionArgs) -> Result<Pagination<Cursor, ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn find_by_semver(
        &self,
        thread_id: Uuid,
        semver: String,
    ) -> Result<Option<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(and(&[eq("thread_id", &thread_id), eq("semver", &semver)]))
            .first()
            .await
    }

    async fn find_latest(&self, thread_id: Uuid) -> Result<Option<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(and(&[eq("thread_id", &thread_id)]))
            .order_by(desc("version"))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ThreadVersion>> {
        self.db
            .bind::<ThreadVersion>()
            .where_by(and(&[in_list("id", &ids)]))
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
            thread_id: input.thread_id,
            version: input.version,
            semver: input.semver,
            status: ThreadVersionStatus::Draft,
            document: None,
            release_note: None,
            description: None,
            owner_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).await
    }

    async fn update_by_id(
        &self,
        id: Uuid,
        input: ThreadVersionUpdateInput,
    ) -> Result<Option<ThreadVersion>> {
        info!("update thread_version: id: {}, input: {:?}", id, input);
        self.db
            .update(&input)
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<ThreadVersion>> {
        self.db
            .delete()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn release(&self, _id: Uuid) -> Result<ThreadVersion> {
        // TODO: copy parameters and save it to thread_version
        todo!()
    }
}

impl From<ThreadVersionService> for ThreadVersionServiceDyn {
    fn from(value: ThreadVersionService) -> Self {
        Arc::new(value) as Self
    }
}
