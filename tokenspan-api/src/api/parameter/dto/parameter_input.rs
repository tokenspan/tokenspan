use crate::api::models::Parameter;
use async_graphql::InputObject;
use rabbit_macros::UpdateModel;
use serde::Serialize;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(InputObject, TypedBuilder, Serialize, Clone, Debug)]
pub struct ParameterCreateInput {
    #[builder(default = "untitled".to_string())]
    pub name: String,
    #[builder(default = 1.0)]
    pub temperature: f32,
    #[builder(default = 256)]
    pub max_tokens: i32,
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
    pub model_id: Uuid,
    pub task_version_id: Uuid,
}

impl From<ParameterCreateInput> for Parameter {
    fn from(value: ParameterCreateInput) -> Self {
        Parameter {
            id: Default::default(),
            name: value.name,
            temperature: value.temperature,
            max_tokens: value.max_tokens,
            stop_sequences: value.stop_sequences,
            top_p: value.top_p,
            frequency_penalty: value.frequency_penalty,
            presence_penalty: value.presence_penalty,
            extra: value.extra,
            model_id: value.model_id,
            task_version_id: value.task_version_id,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

impl From<Parameter> for ParameterCreateInput {
    fn from(value: Parameter) -> Self {
        ParameterCreateInput {
            name: value.name,
            temperature: value.temperature,
            max_tokens: value.max_tokens,
            stop_sequences: value.stop_sequences,
            top_p: value.top_p,
            frequency_penalty: value.frequency_penalty,
            presence_penalty: value.presence_penalty,
            extra: value.extra,
            model_id: value.model_id,
            task_version_id: value.task_version_id,
        }
    }
}

#[derive(InputObject, TypedBuilder, Serialize, Clone, UpdateModel)]
pub struct ParameterUpdateInput {
    #[builder(setter(strip_option))]
    pub name: Option<String>,
    #[builder(setter(strip_option))]
    pub temperature: Option<f32>,
    #[builder(setter(strip_option))]
    pub max_tokens: Option<i32>,
    #[builder(setter(strip_option))]
    pub stop_sequences: Option<Vec<String>>,
    #[builder(setter(strip_option))]
    pub top_p: Option<f32>,
    #[builder(setter(strip_option))]
    pub frequency_penalty: Option<f32>,
    #[builder(setter(strip_option))]
    pub presence_penalty: Option<f32>,
    #[builder(setter(strip_option))]
    pub extra: Option<serde_json::Value>,
    pub model_id: Option<Uuid>,
}
