use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tokenspan_api::api::dto::{
    MessageCreateInput, ParameterCreateInput, ThreadCreateInput, ThreadVersionCreateInput,
};
use tokenspan_api::api::models::ThreadVersionStatus;
use tokio_stream::StreamExt;
use uuid::Uuid;

use tokenspan_api::configs::AppConfig;
use tokenspan_api::state::AppState;

use crate::seed::prelude::{ModelRef, User, UserRef};
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
pub struct ThreadVersion {
    pub owner: UserRef,
    pub semver: String,
    pub version: i32,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub status: ThreadVersionStatus,
    pub release_at: Option<DateTime<Utc>>,
    pub messages: Vec<MessageCreateInput>,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Thread {
    pub owner: UserRef,
    pub name: String,
    pub slug: String,
    pub private: bool,
    pub versions: Vec<ThreadVersion>,
}

pub struct ThreadSeed {
    pub data: Vec<Thread>,
    pub config: AppConfig,
    pub state: AppState,
}

impl ThreadSeed {
    async fn save_messages() {}

    async fn save_parameters(
        &self,
        parameters: Vec<Parameter>,
        thread_version_id: Uuid,
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
                thread_version_id,
            };
            let parameter = parameter_service.create(input).await?;
            println!("Parameter: {} created", parameter.name);
        }

        Ok(())
    }
    async fn save_thread_versions(
        &self,
        thread_id: Uuid,
        thread_versions: Vec<ThreadVersion>,
    ) -> anyhow::Result<()> {
        let thread_version_service = self.state.thread_version_service.clone();
        let user_service = self.state.user_service.clone();

        println!("ThreadVersion: {} creating", thread_versions.len());

        let mut stream = tokio_stream::iter(thread_versions);

        while let Some(thread_version) = stream.next().await {
            let owner = user_service
                .find_by_email(thread_version.owner.email.clone())
                .await?
                .ok_or(anyhow::anyhow!("User not found"))?;

            let input = ThreadVersionCreateInput {
                thread_id,
                semver: thread_version.semver,
                version: thread_version.version,
                release_note: thread_version.release_note,
                description: thread_version.description,
                document: thread_version.document,
            };
            let result = thread_version_service.create(input, owner.id).await?;
            println!("ThreadVersion: {} created", result.version);

            self.save_parameters(thread_version.parameters, result.id)
                .await?;
        }

        Ok(())
    }
}

impl ThreadSeed {
    pub async fn new(config: AppConfig, state: AppState) -> anyhow::Result<Self> {
        let data = Self::load().await?;
        Ok(Self {
            data,
            config,
            state,
        })
    }

    async fn new_with_data(
        config: AppConfig,
        state: AppState,
        data: Vec<Thread>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            data,
            config,
            state,
        })
    }
}

#[async_trait]
impl Seed for ThreadSeed {
    async fn save(&self) -> anyhow::Result<()> {
        let thread_service = self.state.thread_service.clone();
        let user_service = self.state.user_service.clone();

        let mut stream = tokio_stream::iter(self.data.clone());
        while let Some(thread) = stream.next().await {
            let result = thread_service.find_by_slug(thread.slug.clone()).await?;
            if let Some(thread) = result {
                println!("Thread: {} already existed", thread.name);
                continue;
            }

            let owner = user_service
                .find_by_email(thread.owner.email.clone())
                .await?
                .ok_or(anyhow::anyhow!("User not found"))?;

            let versions = thread.versions;
            let thread = thread_service
                .create(
                    ThreadCreateInput {
                        name: thread.name,
                        slug: thread.slug,
                        private: thread.private,
                    },
                    owner.id,
                )
                .await?;

            self.save_thread_versions(thread.id, versions).await?;
            println!("Thread: {} created", thread.name)
        }

        Ok(())
    }

    fn path() -> &'static str {
        "./seed/threads"
    }
}
