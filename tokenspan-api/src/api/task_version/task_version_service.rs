use std::sync::Arc;

use async_graphql::Result;
use prisma_client_rust::Direction;

use crate::api::models::{TaskId, TaskVersionId, UserId};
use crate::api::task_version::dto::{
    CreateTaskVersionInput, TaskVersionArgs, UpdateTaskVersionInput,
};
use crate::api::task_version::task_version_error::TaskVersionError;
use crate::api::task_version::task_version_model::TaskVersion;
use crate::prisma::{task, task_version, user, PrismaClient};
use tokenspan_utils::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait TaskVersionServiceExt {
    async fn get_task_versions(
        &self,
        args: TaskVersionArgs,
    ) -> Result<Pagination<Cursor, TaskVersion>>;
    async fn get_task_version_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>>;
    async fn get_task_versions_by_ids(&self, ids: Vec<TaskVersionId>) -> Result<Vec<TaskVersion>>;
    async fn get_task_versions_by_task_id(&self, task_id: TaskId) -> Result<Vec<TaskVersion>>;
    async fn count_task_versions(&self) -> Result<i64>;
    async fn create_task_version(
        &self,
        input: CreateTaskVersionInput,
        owner: &UserId,
    ) -> Result<TaskVersion>;
    async fn update_task_version(
        &self,
        id: TaskVersionId,
        input: UpdateTaskVersionInput,
    ) -> Result<TaskVersion>;
    async fn delete_task_version(&self, id: TaskVersionId) -> Result<TaskVersion>;
    async fn release_task_version(&self, id: TaskVersionId) -> Result<TaskVersion>;
}

pub type TaskVersionServiceDyn = Arc<dyn TaskVersionServiceExt + Send + Sync>;

pub struct TaskVersionService {
    prisma: Arc<PrismaClient>,
}

impl TaskVersionService {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }
}

#[async_trait::async_trait]
impl TaskVersionServiceExt for TaskVersionService {
    async fn get_task_versions(
        &self,
        args: TaskVersionArgs,
    ) -> Result<Pagination<Cursor, TaskVersion>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .task_version()
            .find_many(vec![])
            .take(take + 1)
            .order_by(task_version::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take(-(take + 2))
                .cursor(task_version::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(task_version::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersions)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_task_version_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>> {
        let task_version = self
            .prisma
            .task_version()
            .find_unique(task_version::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersion)?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn get_task_versions_by_ids(&self, ids: Vec<TaskVersionId>) -> Result<Vec<TaskVersion>> {
        let ids = ids
            .into_iter()
            .map(|id| task_version::id::equals(id.into()))
            .collect();
        let task_versions = self
            .prisma
            .task_version()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersions)?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect();

        Ok(task_versions)
    }

    async fn get_task_versions_by_task_id(&self, task_id: TaskId) -> Result<Vec<TaskVersion>> {
        let task_versions = self
            .prisma
            .task_version()
            .find_many(vec![task_version::task_id::equals(task_id.into())])
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersions)?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect();

        Ok(task_versions)
    }

    async fn count_task_versions(&self) -> Result<i64> {
        let count = self
            .prisma
            .task_version()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToCountTaskVersions)?;

        Ok(count)
    }

    async fn create_task_version(
        &self,
        input: CreateTaskVersionInput,
        owner: &UserId,
    ) -> Result<TaskVersion> {
        let created_task_version = self
            .prisma
            .task_version()
            .create(
                user::id::equals(owner.to_string()),
                task::id::equals(input.task_id.into()),
                input.version,
                vec![
                    task_version::release_note::set(input.release_note),
                    task_version::description::set(input.description),
                    task_version::document::set(input.document),
                    task_version::messages::set(input.messages),
                    task_version::status::set(input.status),
                ],
            )
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToCreateTaskVersion)?;

        Ok(created_task_version.into())
    }

    async fn update_task_version(
        &self,
        id: TaskVersionId,
        input: UpdateTaskVersionInput,
    ) -> Result<TaskVersion> {
        let updated_task_version = self
            .prisma
            .task_version()
            .update(task_version::id::equals(id.into()), input.into())
            .exec()
            .await
            .map_err(|_| TaskVersionError::UnableToUpdateTaskVersion)?;

        Ok(updated_task_version.into())
    }

    async fn delete_task_version(&self, id: TaskVersionId) -> Result<TaskVersion> {
        let deleted_task_version = self
            .prisma
            .task_version()
            .delete(task_version::id::equals(id.into()))
            .exec()
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
