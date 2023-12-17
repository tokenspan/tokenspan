use async_graphql::InputObject;
use serde::Serialize;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(InputObject, TypedBuilder, Serialize)]
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
    pub model_id: Uuid,
}
