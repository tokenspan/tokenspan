use async_graphql::InputObject;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(InputObject, TypedBuilder)]
pub struct ParameterCreateInput {
    #[builder(default = "untitled".to_string())]
    pub name: String,
    #[builder(default = 1.0)]
    pub temperature: f32,
    #[builder(default = 256)]
    pub max_tokens: u32,
    #[builder(default)]
    pub stop_sequences: Vec<String>,
    #[builder(default = 1.0)]
    pub top_p: f32,
    #[builder(default = 0.0)]
    pub frequency_penalty: f32,
    #[builder(default = 0.0)]
    pub presence_penalty: f32,
    #[builder(default)]
    pub extra: Option<serde_json::Value>,
    pub task_version_id: Uuid,
    pub model_id: Uuid,
}

impl TryFrom<ParameterUpsertInput> for ParameterCreateInput {
    type Error = anyhow::Error;

    fn try_from(input: ParameterUpsertInput) -> Result<Self, Self::Error> {
        Ok(Self {
            name: input.name.ok_or(anyhow::anyhow!("name is required"))?,
            temperature: input
                .temperature
                .ok_or(anyhow::anyhow!("temperature is required"))?,
            max_tokens: input
                .max_tokens
                .ok_or(anyhow::anyhow!("max_tokens is required"))?,
            stop_sequences: input
                .stop_sequences
                .ok_or(anyhow::anyhow!("stop_sequences is required"))?,
            top_p: input.top_p.ok_or(anyhow::anyhow!("top_p is required"))?,
            frequency_penalty: input
                .frequency_penalty
                .ok_or(anyhow::anyhow!("frequency_penalty is required"))?,
            presence_penalty: input
                .presence_penalty
                .ok_or(anyhow::anyhow!("presence_penalty is required"))?,
            extra: input.extra,
            task_version_id: input.task_version_id,
            model_id: input
                .model_id
                .ok_or(anyhow::anyhow!("model_id is required"))?,
        })
    }
}

#[derive(InputObject, Clone)]
pub struct ParameterUpdateInput {
    pub name: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub extra: Option<serde_json::Value>,
    #[graphql(skip)]
    pub task_version_id: Uuid,
    pub model_id: Option<Uuid>,
}

impl ParameterUpdateInput {
    pub fn copy(&self, model: &mut entity::parameter::ActiveModel) {
        if let Some(ref name) = self.name {
            model.name = Set(name.clone());
        }

        if let Some(ref model_id) = self.model_id {
            model.model_id = Set(model_id.clone().into());
        }

        if let Some(max_tokens) = self.max_tokens {
            model.max_tokens = Set(max_tokens as i32);
        }

        if let Some(temperature) = self.temperature {
            model.temperature = Set(temperature);
        }

        if let Some(ref stop_sequences) = self.stop_sequences {
            model.stop_sequences = Set(stop_sequences.clone());
        }

        if let Some(top_p) = self.top_p {
            model.top_p = Set(top_p);
        }

        if let Some(frequency_penalty) = self.frequency_penalty {
            model.frequency_penalty = Set(frequency_penalty);
        }

        if let Some(presence_penalty) = self.presence_penalty {
            model.presence_penalty = Set(presence_penalty);
        }

        model.extra = Set(self.extra.clone());
        model.updated_at = Set(Utc::now().naive_utc());
    }
}

impl From<ParameterUpsertInput> for ParameterUpdateInput {
    fn from(input: ParameterUpsertInput) -> Self {
        Self {
            name: input.name,
            temperature: input.temperature,
            max_tokens: input.max_tokens,
            stop_sequences: input.stop_sequences,
            top_p: input.top_p,
            frequency_penalty: input.frequency_penalty,
            presence_penalty: input.presence_penalty,
            extra: input.extra,
            model_id: input.model_id,
            task_version_id: input.task_version_id,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct ParameterUpsertInput {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub extra: Option<serde_json::Value>,
    #[graphql(skip)]
    pub task_version_id: Uuid,
    pub model_id: Option<Uuid>,
}
