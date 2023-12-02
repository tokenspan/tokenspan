use crate::api::models::{ModelId, TaskVersionId};
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ParameterCreateInput {
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub task_version_id: TaskVersionId,
    pub model_id: ModelId,
}

#[derive(InputObject)]
pub struct ParameterUpdateInput {
    pub name: Option<String>,
    pub model_id: Option<ModelId>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub extra: Option<serde_json::Value>,
}
