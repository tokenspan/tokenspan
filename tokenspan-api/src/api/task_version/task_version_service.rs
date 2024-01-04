use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::ops::{and, desc, eq, in_list};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
use tracing::info;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::models::{TaskVersion, TaskVersionStatus};
use crate::api::task_version::dto::{
    TaskVersionArgs, TaskVersionCreateInput, TaskVersionUpdateInput,
};

#[async_trait::async_trait]
pub trait TaskVersionServiceExt {
    async fn paginate(&self, args: TaskVersionArgs) -> Result<Pagination<Cursor, TaskVersion>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskVersion>>;
    async fn find_by_semver(&self, task_id: Uuid, semver: String) -> Result<Option<TaskVersion>>;
    async fn find_latest(&self, task_id: Uuid) -> Result<Option<TaskVersion>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<TaskVersion>>;
    async fn create(&self, input: TaskVersionCreateInput, owner_id: Uuid) -> Result<TaskVersion>;
    async fn update_by_id(
        &self,
        id: Uuid,
        input: TaskVersionUpdateInput,
    ) -> Result<Option<TaskVersion>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<TaskVersion>>;
    async fn release(&self, id: Uuid) -> Result<TaskVersion>;
}

pub type TaskVersionServiceDyn = Arc<dyn TaskVersionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct TaskVersionService {
    db: Database,
}

#[async_trait::async_trait]
impl TaskVersionServiceExt for TaskVersionService {
    async fn paginate(&self, args: TaskVersionArgs) -> Result<Pagination<Cursor, TaskVersion>> {
        self.db
            .bind::<TaskVersion>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskVersion>> {
        self.db
            .bind::<TaskVersion>()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn find_by_semver(&self, task_id: Uuid, semver: String) -> Result<Option<TaskVersion>> {
        self.db
            .bind::<TaskVersion>()
            .where_by(and(&[eq("task_id", &task_id), eq("semver", &semver)]))
            .first()
            .await
    }

    async fn find_latest(&self, task_id: Uuid) -> Result<Option<TaskVersion>> {
        self.db
            .bind::<TaskVersion>()
            .where_by(and(&[eq("task_id", &task_id)]))
            .order_by(desc("version"))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<TaskVersion>> {
        self.db
            .bind::<TaskVersion>()
            .where_by(and(&[in_list("id", &ids)]))
            .all()
            .await
    }

    async fn create(&self, input: TaskVersionCreateInput, owner_id: Uuid) -> Result<TaskVersion> {
        let messages = input
            .messages
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        let input = TaskVersion {
            id: Uuid::new_v4(),
            task_id: input.task_id,
            version: input.version,
            semver: input.semver,
            status: TaskVersionStatus::Draft,
            document: None,
            release_note: None,
            description: None,
            owner_id,
            messages,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).await
    }

    async fn update_by_id(
        &self,
        id: Uuid,
        input: TaskVersionUpdateInput,
    ) -> Result<Option<TaskVersion>> {
        info!("update task_version: id: {}, input: {:?}", id, input);
        self.db
            .update(&input)
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<TaskVersion>> {
        self.db
            .delete()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn release(&self, _id: Uuid) -> Result<TaskVersion> {
        // TODO: copy parameters and save it to task_version
        todo!()
    }
}

impl From<TaskVersionService> for TaskVersionServiceDyn {
    fn from(value: TaskVersionService) -> Self {
        Arc::new(value) as Self
    }
}
