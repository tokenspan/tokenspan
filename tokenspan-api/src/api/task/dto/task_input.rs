use crate::api::models::{ApiKeyId, ParameterId, TaskVersionId};
use async_graphql::InputObject;

use crate::prisma::task;

#[derive(InputObject)]
pub struct CreateTaskInput {
    pub name: String,
    pub slug: String,
}

#[derive(InputObject)]
pub struct UpdateTaskInput {
    pub name: Option<String>,
}

impl From<UpdateTaskInput> for Vec<task::SetParam> {
    fn from(value: UpdateTaskInput) -> Self {
        let mut params = Vec::new();
        if let Some(name) = value.name {
            params.push(task::name::set(name));
        }

        params
    }
}

#[derive(InputObject)]
pub struct ExecuteTaskInput {
    pub task_version_id: TaskVersionId,
    pub parameter_id: ParameterId,
    pub api_key_id: ApiKeyId,
}
