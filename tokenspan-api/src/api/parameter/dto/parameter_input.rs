use async_graphql::InputObject;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
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
    pub task_version_id: Uuid,
}

#[derive(InputObject, TypedBuilder, Serialize, Clone)]
pub struct ParameterUpdateInput {
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

impl ParameterUpdateInput {
    pub fn copy(self, model: &mut entity::parameter::ActiveModel) {
        if let Some(name) = self.name {
            model.name = Set(name);
        }

        if let Some(temperature) = self.temperature {
            model.temperature = Set(temperature);
        }

        if let Some(max_tokens) = self.max_tokens {
            model.max_tokens = Set(max_tokens as i32);
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

        if let Some(model_id) = self.model_id {
            model.model_id = Set(model_id);
        }

        model.extra = Set(self.extra.clone());
        model.updated_at = Set(Utc::now().naive_utc());
    }
}
