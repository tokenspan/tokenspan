use async_graphql::{InputObject, OneofObject};

use crate::api::models::{ModelId, ParameterId};

#[derive(OneofObject)]
pub enum ParameterInputBy {
    Create(ParameterCreateInput),
    Update(ParameterUpdateInput),
    Delete(ParameterDeleteInput),
}

#[derive(InputObject)]
pub struct ParameterDeleteInput {
    pub id: ParameterId,
}

#[derive(InputObject)]
pub struct ParameterCreateInput {
    pub data: ParameterInput,
}

#[derive(InputObject)]
pub struct ParameterUpdateInput {
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
