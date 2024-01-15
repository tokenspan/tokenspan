use async_graphql::InputObject;
use dojo_macros::UpdateModel;
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
    #[builder(default = false)]
    pub is_default: bool,
    pub model_id: Uuid,
    pub thread_version_id: Uuid,
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
