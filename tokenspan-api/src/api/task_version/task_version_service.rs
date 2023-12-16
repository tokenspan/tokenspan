use std::sync::Arc;

use anyhow::Result;
use bson::doc;
use bson::oid::ObjectId;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::dto::ParameterInputBy;
use crate::api::models::{TaskId, TaskVersion, TaskVersionId, UserId};
use crate::api::repositories::{
    ParameterEntity, TaskVersionCreateEntity, TaskVersionStatus, TaskVersionUpdateEntity,
};
use crate::api::task_version::dto::{
    TaskVersionArgs, TaskVersionCreateInput, TaskVersionUpdateInput,
};
use crate::api::task_version::task_version_error::TaskVersionError;
use crate::prompt::ChatMessage;
use crate::repository::RootRepository;

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
    async fn find_by_task_id(&self, task_id: TaskId) -> Result<Vec<TaskVersion>>;
    async fn count(&self) -> Result<u64>;
    async fn create(&self, input: TaskVersionCreateInput, owner: &UserId) -> Result<TaskVersion>;
    async fn update_by_id(
        &self,
        id: TaskVersionId,
        input: TaskVersionUpdateInput,
    ) -> Result<Option<TaskVersion>>;
    async fn delete_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>>;
    async fn release(&self, id: TaskVersionId) -> Result<TaskVersion>;
}

pub type TaskVersionServiceDyn = Arc<dyn TaskVersionServiceExt + Send + Sync>;

pub struct TaskVersionService {
    repository: RootRepository,
}

impl TaskVersionService {
    pub fn new(repository: RootRepository) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl TaskVersionServiceExt for TaskVersionService {
    async fn paginate(&self, args: TaskVersionArgs) -> Result<Pagination<Cursor, TaskVersion>> {
        let task_id = ObjectId::from(args.task_id.clone());
        let paginated = self
            .repository
            .task_version
            .paginate_with_filter::<TaskVersion>(
                doc! {
                    "taskId": task_id,
                },
                args.into(),
            )
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(paginated)
    }

    async fn find_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>> {
        let task_version = self
            .repository
            .task_version
            .find_by_id(id)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_by_version(
        &self,
        task_id: TaskId,
        version: String,
    ) -> Result<Option<TaskVersion>> {
        let task_version = self
            .repository
            .task_version
            .find_by_version(task_id, version)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_latest(&self, task_id: TaskId) -> Result<Option<TaskVersion>> {
        let task_version = self
            .repository
            .task_version
            .find_latest(task_id)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_by_ids(&self, ids: Vec<TaskVersionId>) -> Result<Vec<TaskVersion>> {
        let task_versions = self
            .repository
            .task_version
            .find_many_by_ids(ids)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect();

        Ok(task_versions)
    }

    async fn find_by_task_id(&self, task_id: TaskId) -> Result<Vec<TaskVersion>> {
        let task_versions = self
            .repository
            .task_version
            .find_by_task_id(task_id)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect();

        Ok(task_versions)
    }

    async fn count(&self) -> Result<u64> {
        let count = self
            .repository
            .task_version
            .count()
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(count)
    }

    async fn create(&self, input: TaskVersionCreateInput, owner: &UserId) -> Result<TaskVersion> {
        let messages = input
            .messages
            .into_iter()
            .map(|message| message.into())
            .collect();

        let parameters = input
            .parameters
            .into_iter()
            .map(|p| p.data.into())
            .collect();

        println!("parameters: {:?}", parameters);

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
                status: TaskVersionStatus::Draft,
                parameters,
                messages,
            })
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_task_version.into())
    }

    async fn update_by_id(
        &self,
        id: TaskVersionId,
        input: TaskVersionUpdateInput,
    ) -> Result<Option<TaskVersion>> {
        let messages: Option<Vec<ChatMessage>> = input
            .messages
            .map(|messages| messages.into_iter().map(|message| message.into()).collect());

        let task_version = self
            .find_by_id(id.clone())
            .await?
            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                "task version not found"
            )))?;

        let mut parameters: Vec<ParameterEntity> = task_version
            .parameters
            .into_iter()
            .map(|p| p.into())
            .collect();
        if let Some(inputs) = input.parameters {
            for input in inputs {
                match input {
                    ParameterInputBy::Create(input) => parameters.push(input.data.into()),
                    ParameterInputBy::Update(input) => {
                        let index = parameters.iter().position(|p| p.id == input.id);
                        if let Some(index) = index {
                            parameters[index] = ParameterEntity::new_with_id(input.id, input.data);
                        }
                    }
                    ParameterInputBy::Delete(input) => {
                        let index = parameters.iter().position(|p| p.id == input.id);
                        if let Some(index) = index {
                            parameters.remove(index);
                        }
                    }
                }
            }
        }

        let updated_task_version = self
            .repository
            .task_version
            .update_by_id(
                id,
                TaskVersionUpdateEntity {
                    status: None,
                    release_note: input.release_note,
                    description: input.description,
                    document: input.document,
                    messages,
                    parameters,
                },
            )
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(updated_task_version)
    }

    async fn delete_by_id(&self, id: TaskVersionId) -> Result<Option<TaskVersion>> {
        let deleted_task_version = self
            .repository
            .task_version
            .delete_by_id(id)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(deleted_task_version)
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
