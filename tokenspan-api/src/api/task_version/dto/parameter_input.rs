use async_graphql::{InputObject, OneofObject};
use chrono::{DateTime, Utc};

use crate::api::models::{ModelId, ParameterId};

#[derive(OneofObject)]
pub enum ParameterInputBy {
    Create(CreateParameterInput),
    Update(UpdateParameterInput),
    Delete(DeleteParameterInput),
}

#[derive(InputObject)]
pub struct DeleteParameterInput {
    pub id: ParameterId,
}

#[derive(InputObject)]
pub struct CreateParameterInput {
    pub data: ParameterInput,
}

#[derive(InputObject)]
pub struct UpdateParameterInput {
    pub id: ParameterId,
    pub data: ParameterInput,
}

#[derive(InputObject)]
pub struct ParameterInput {
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u16,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub model_id: ModelId,
}
