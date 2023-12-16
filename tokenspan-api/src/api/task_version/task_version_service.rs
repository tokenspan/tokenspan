use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use entity::sea_orm_active_enums::TaskVersionStatus;
use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::models::{TaskId, TaskVersion, TaskVersionId, UserId};
use crate::api::task_version::dto::{
    TaskVersionArgs, TaskVersionCreateInput, TaskVersionUpdateInput,
};
use crate::api::task_version::task_version_error::TaskVersionError;

#[async_trait::async_trait]
pub trait TaskVersionServiceExt {
    async fn paginate(&self, args: TaskVersionArgs) -> Result<Pagination<Cursor, TaskVersion>>;
    async fn find_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>>;
    async fn find_by_version(
        &self,
        task_id: TaskId,
        version: String,
    ) -> Result<Option<TaskVersion>>;
    async fn find_latest(&self, task_id: TaskId) -> Result<Option<TaskVersion>>;
    async fn find_by_ids(&self, ids: Vec<TaskVersionId>) -> Result<Vec<TaskVersion>>;
    async fn create(&self, input: TaskVersionCreateInput, owner_id: UserId) -> Result<TaskVersion>;
    async fn update_by_id(
        &self,
        id: TaskVersionId,
        input: TaskVersionUpdateInput,
    ) -> Result<TaskVersion>;
    async fn delete_by_id(&self, id: TaskVersionId) -> Result<TaskVersion>;
    async fn release(&self, id: TaskVersionId) -> Result<TaskVersion>;
}

pub type TaskVersionServiceDyn = Arc<dyn TaskVersionServiceExt + Send + Sync>;

pub struct TaskVersionService {
    db: DatabaseConnection,
}

impl TaskVersionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl TaskVersionServiceExt for TaskVersionService {
    async fn paginate(&self, args: TaskVersionArgs) -> Result<Pagination<Cursor, TaskVersion>> {
        let take = args.take.unwrap_or(10) as u64;
        let mut cursor = entity::task_version::Entity::find()
            .cursor_by(entity::task_version::Column::Id)
            .order_by_desc(entity::task_version::Column::Id)
            .limit(Some(take));

        if let Some(after) = args.after.clone() {
            cursor.after(after.id);
        }

        if let Some(before) = args.before.clone() {
            cursor.before(before.id);
        }

        let count = entity::task_version::Entity::find().count(&self.db).await?;
        let items = cursor
            .all(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(
            items,
            args.before,
            args.after,
            take as i64,
            count,
        ))
    }

    async fn find_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>> {
        let task_version = entity::task_version::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_by_version(
        &self,
        task_id: TaskId,
        semver: String,
    ) -> Result<Option<TaskVersion>> {
        let task_version = entity::task_version::Entity::find()
            .filter(
                entity::task_version::Column::TaskId
                    .eq(task_id)
                    .and(entity::task_version::Column::Semver.eq(semver)),
            )
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_latest(&self, task_id: TaskId) -> Result<Option<TaskVersion>> {
        let task_version = entity::task_version::Entity::find()
            .filter(entity::task_version::Column::TaskId.eq(task_id))
            .order_by_desc(entity::api_key::Column::Id)
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_by_ids(&self, ids: Vec<TaskVersionId>) -> Result<Vec<TaskVersion>> {
        let ids = ids.into_iter().map(|id| id.to_string()).collect::<Vec<_>>();
        let task_versions = entity::task_version::Entity::find()
            .filter(entity::task_version::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect::<Vec<_>>();

        Ok(task_versions)
    }

    async fn create(&self, input: TaskVersionCreateInput, owner_id: UserId) -> Result<TaskVersion> {
        let created_task_version = entity::task_version::ActiveModel {
            id: Set(TaskVersionId::new_v4()),
            task_id: Set(input.task_id.into()),
            owner_id: Set(owner_id.into()),
            semver: Set(input.semver),
            version: Set(input.version as i32),
            description: Set(input.description),
            release_note: Set(input.release_note),
            document: Set(input.document),
            status: Set(TaskVersionStatus::Draft),
            released_at: Set(None),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_task_version)
    }

    async fn update_by_id(
        &self,
        id: TaskVersionId,
        input: TaskVersionUpdateInput,
    ) -> Result<TaskVersion> {
        let mut task_version = entity::task_version::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                "TaskVersion not found"
            )))?
            .into_active_model();

        input.merge(&mut task_version);

        let updated_task_version = task_version
            .update(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into();

        Ok(updated_task_version)
    }

    async fn delete_by_id(&self, id: TaskVersionId) -> Result<TaskVersion> {
        let deleted_task_version = entity::task_version::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                "TaskVersion not found"
            )))?;

        deleted_task_version
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(deleted_task_version.into())
    }

    async fn release(&self, _id: TaskVersionId) -> Result<TaskVersion> {
        // TODO: copy parameters and save it to task_version
        todo!()
    }
}

impl From<TaskVersionService> for TaskVersionServiceDyn {
    fn from(value: TaskVersionService) -> Self {
        Arc::new(value) as Self
    }
}
