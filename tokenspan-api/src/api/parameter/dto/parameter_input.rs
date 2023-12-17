use async_graphql::{InputObject, OneofObject};
use serde::Serialize;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(InputObject, TypedBuilder, Serialize, Clone, Debug)]
pub struct ParameterCreateInput {
    #[graphql(skip, default_with = "Uuid::new_v4()")]
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
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

#[derive(InputObject, TypedBuilder, Serialize, Clone)]
pub struct ParameterUpdateInput {
    pub id: Uuid,
    #[builder(setter(strip_option))]
    pub name: Option<String>,
    #[builder(setter(strip_option))]
    pub temperature: Option<f32>,
    #[builder(setter(strip_option))]
    pub max_tokens: Option<u32>,
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

#[derive(OneofObject)]
pub enum ParameterMutationInput {
    Create(ParameterCreateInput),
    Update(ParameterUpdateInput),
    Delete(Uuid),
}
