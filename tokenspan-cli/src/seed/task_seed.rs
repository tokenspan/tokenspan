use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tokenspan_api::api::dto::{
    MessageCreateInput, ParameterCreateInput, TaskCreateInput, TaskVersionCreateInput,
};
use tokenspan_api::api::models::TaskVersionStatus;
use tokio_stream::StreamExt;
use uuid::Uuid;

use tokenspan_api::configs::AppConfig;
use tokenspan_api::state::AppState;

use crate::seed::prelude::{ModelRef, UserRef};
use crate::seed::Seed;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub model: ModelRef,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskVersion {
    pub owner: UserRef,
    pub semver: String,
    pub version: i32,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub status: TaskVersionStatus,
    pub release_at: Option<DateTime<Utc>>,
    pub messages: Vec<MessageCreateInput>,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub owner: UserRef,
    pub name: String,
    pub slug: String,
    pub private: bool,
    pub versions: Vec<TaskVersion>,
}

pub struct TaskSeed {
    pub data: Vec<Task>,
    pub config: AppConfig,
    pub state: AppState,
}

impl TaskSeed {
    async fn save_parameters(
        &self,
        parameters: Vec<Parameter>,
        task_version_id: Uuid,
    ) -> anyhow::Result<()> {
        let model_service = self.state.model_service.clone();
        let parameter_service = self.state.parameter_service.clone();

        let mut stream = tokio_stream::iter(parameters);

        while let Some(parameter) = stream.next().await {
            let model = model_service
                .find_by_slug(parameter.model.slug.clone())
                .await?
                .ok_or(anyhow::anyhow!("Model not found"))?;

            let input = ParameterCreateInput {
                name: parameter.name,
                temperature: parameter.temperature,
                max_tokens: parameter.max_tokens,
                stop_sequences: parameter.stop_sequences,
                top_p: parameter.top_p,
                frequency_penalty: parameter.frequency_penalty,
                presence_penalty: parameter.presence_penalty,
                extra: parameter.extra,
                model_id: model.id,
                task_version_id,
            };
            let parameter = parameter_service.create(input).await?;
            println!("Parameter: {} created", parameter.name);
        }

        Ok(())
    }
    async fn save_task_versions(
        &self,
        task_id: Uuid,
        task_versions: Vec<TaskVersion>,
    ) -> anyhow::Result<()> {
        let task_version_service = self.state.task_version_service.clone();
        let user_service = self.state.user_service.clone();

        println!("TaskVersion: {} creating", task_versions.len());

        let mut stream = tokio_stream::iter(task_versions);

        while let Some(task_version) = stream.next().await {
            let owner = user_service
                .find_by_email(task_version.owner.email.clone())
                .await?
                .ok_or(anyhow::anyhow!("User not found"))?;

            let input = TaskVersionCreateInput {
                task_id,
                messages: task_version.messages,
                semver: task_version.semver,
                version: task_version.version,
                release_note: task_version.release_note,
                description: task_version.description,
                document: task_version.document,
            };
            let result = task_version_service.create(input, owner.id).await?;
            println!("TaskVersion: {} created", result.version);

            self.save_parameters(task_version.parameters, result.id)
                .await?;
        }

        Ok(())
    }
}

#[async_trait]
impl Seed for TaskSeed {
    async fn new(config: AppConfig, state: AppState) -> anyhow::Result<Self> {
        let data = Self::load().await?;
        Ok(Self {
            data,
            config,
            state,
        })
    }

    async fn save(&self) -> anyhow::Result<()> {
        let task_service = self.state.task_service.clone();
        let user_service = self.state.user_service.clone();

        let mut stream = tokio_stream::iter(self.data.clone());
        while let Some(task) = stream.next().await {
            let result = task_service.find_by_slug(task.slug.clone()).await?;
            if let Some(task) = result {
                println!("Task: {} already existed", task.name);
                continue;
            }

            let owner = user_service
                .find_by_email(task.owner.email.clone())
                .await?
                .ok_or(anyhow::anyhow!("User not found"))?;

            let versions = task.versions;
            let task = task_service
                .create(
                    TaskCreateInput {
                        name: task.name,
                        slug: task.slug,
                        private: task.private,
                    },
                    owner.id,
                )
                .await?;

            self.save_task_versions(task.id, versions).await?;
            println!("Task: {} created", task.name)
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/tasks"
    }
}
