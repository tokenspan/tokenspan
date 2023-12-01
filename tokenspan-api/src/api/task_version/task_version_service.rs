use std::sync::Arc;

use async_graphql::Result;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::models::{TaskId, TaskVersionId, UserId};
use crate::api::repositories::{
    TaskVersionCreateEntity, TaskVersionStatus, TaskVersionUpdateEntity,
};
use crate::api::task_version::dto::{
    TaskVersionArgs, TaskVersionCreateInput, TaskVersionUpdateInput,
};
use crate::api::task_version::task_version_error::TaskVersionError;
use crate::api::task_version::task_version_model::TaskVersion;
use crate::repository::RootRepository;

#[async_trait::async_trait]
pub trait TaskVersionServiceExt {
    async fn get_task_versions(
        &self,
        args: TaskVersionArgs,
    ) -> Result<Pagination<Cursor, TaskVersion>>;
    async fn get_task_version_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>>;
    async fn get_task_versions_by_ids(&self, ids: Vec<TaskVersionId>) -> Result<Vec<TaskVersion>>;
    async fn get_task_versions_by_task_id(&self, task_id: TaskId) -> Result<Vec<TaskVersion>>;
    async fn count_task_versions(&self) -> Result<u64>;
    async fn create_task_version(
        &self,
        input: TaskVersionCreateInput,
        owner: &UserId,
    ) -> Result<TaskVersion>;
    async fn update_task_version(
        &self,
        id: TaskVersionId,
        input: TaskVersionUpdateInput,
    ) -> Result<TaskVersion>;
    async fn delete_task_version(&self, id: TaskVersionId) -> Result<Option<TaskVersion>>;
    async fn release_task_version(&self, id: TaskVersionId) -> Result<TaskVersion>;
}

pub type TaskVersionServiceDyn = Arc<dyn TaskVersionServiceExt + Send + Sync>;

pub struct TaskVersionService {
    repository: Arc<RootRepository>,
}

impl TaskVersionService {
    pub fn new(repository: Arc<RootRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl TaskVersionServiceExt for TaskVersionService {
    async fn get_task_versions(
        &self,
        args: TaskVersionArgs,
    ) -> Result<Pagination<Cursor, TaskVersion>> {
        let paginated = self
            .repository
            .view
            .paginate::<TaskVersion>(args.take, args.before, args.after)
            .await
            .map_err(|_| TaskVersionError::UnableToCountTaskVersions)?;

        Ok(paginated)
    }

    async fn get_task_version_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>> {
        let task_version = self
            .repository
            .task_version
            .find_by_id(id)
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersion)?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn get_task_versions_by_ids(&self, ids: Vec<TaskVersionId>) -> Result<Vec<TaskVersion>> {
        let task_versions = self
            .repository
            .task_version
            .find_many_by_ids(ids)
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersions)?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect();

        Ok(task_versions)
    }

    async fn get_task_versions_by_task_id(&self, task_id: TaskId) -> Result<Vec<TaskVersion>> {
        let task_versions = self
            .repository
            .task_version
            .find_by_task_id(task_id)
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersions)?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect();

        Ok(task_versions)
    }

    async fn count_task_versions(&self) -> Result<u64> {
        let count = self
            .repository
            .task_version
            .count()
            .await
            .map_err(|_| TaskVersionError::UnableToCountTaskVersions)?;

        Ok(count)
    }

    async fn create_task_version(
        &self,
        input: TaskVersionCreateInput,
        owner: &UserId,
    ) -> Result<TaskVersion> {
        let created_task_version = self
            .repository
            .task_version
            .create(TaskVersionCreateEntity {
                task_id: input.task_id,
                owner_id: owner.clone(),
                version: input.version,
                release_note: input.release_note,
                description: input.description,
                document: input.document,
                parameters: vec![],
                messages: input.messages,
                status: TaskVersionStatus::Draft,
            })
            .await
            .map_err(|_| TaskVersionError::UnableToCreateTaskVersion)?;

        Ok(created_task_version.into())
    }

    async fn update_task_version(
        &self,
        id: TaskVersionId,
        input: TaskVersionUpdateInput,
    ) -> Result<TaskVersion> {
        let updated_task_version = self
            .repository
            .task_version
            .update_by_id(
                id,
                TaskVersionUpdateEntity {
                    status: input.status,
                    release_note: input.release_note,
                    description: input.description,
                    document: input.document,
                    messages: input.messages,
                },
            )
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToUpdateTaskVersion)?;

        Ok(updated_task_version.into())
    }

    async fn delete_task_version(&self, id: TaskVersionId) -> Result<Option<TaskVersion>> {
        let deleted_task_version = self
            .repository
            .task_version
            .delete_by_id(id)
            .await
            .map_err(|_| TaskVersionError::UnableToDeleteTaskVersion)?;

        Ok(deleted_task_version.into())
    }

    async fn release_task_version(&self, _id: TaskVersionId) -> Result<TaskVersion> {
        // TODO: copy parameters and save it to task_version
        todo!()
    }
}

impl From<TaskVersionService> for TaskVersionServiceDyn {
    fn from(value: TaskVersionService) -> Self {
        Arc::new(value) as Self
    }
}
