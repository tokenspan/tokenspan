use crate::api::models::{ModelId, TaskVersionId};
use crate::prisma::parameter;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateParameterInput {
    pub name: String,
    pub temperature: f64,
    pub max_tokens: i32,
    pub stop_sequences: Vec<String>,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub extra: Option<serde_json::Value>,
    pub task_version_id: TaskVersionId,
    pub model_id: ModelId,
}

#[derive(InputObject)]
pub struct UpdateParameterInput {
    pub name: Option<String>,
    pub model_id: Option<ModelId>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i32>,
    pub stop_sequences: Option<Vec<String>>,
    pub top_p: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub presence_penalty: Option<f64>,
    pub extra: Option<serde_json::Value>,
}

impl From<UpdateParameterInput> for Vec<parameter::SetParam> {
    fn from(value: UpdateParameterInput) -> Self {
        let mut params = Vec::new();
        if let Some(name) = value.name {
            params.push(parameter::name::set(name));
        }

        if let Some(model_id) = value.model_id {
            params.push(parameter::model_id::set(model_id.into()));
        }

        if let Some(temperature) = value.temperature {
            params.push(parameter::temperature::set(temperature));
        }

        if let Some(max_tokens) = value.max_tokens {
            params.push(parameter::max_tokens::set(max_tokens));
        }

        if let Some(stop_sequences) = value.stop_sequences {
            params.push(parameter::stop_sequences::set(stop_sequences));
        }

        if let Some(top_p) = value.top_p {
            params.push(parameter::top_p::set(top_p));
        }

        if let Some(frequency_penalty) = value.frequency_penalty {
            params.push(parameter::frequency_penalty::set(frequency_penalty));
        }

        if let Some(presence_penalty) = value.presence_penalty {
            params.push(parameter::presence_penalty::set(presence_penalty));
        }

        if let Some(extra) = value.extra {
            params.push(parameter::extra::set(Some(extra)));
        }

        params
    }
}
