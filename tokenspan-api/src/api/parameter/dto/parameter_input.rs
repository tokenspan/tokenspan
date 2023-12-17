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

#[derive(InputObject)]
pub struct ParameterUpdateInput {
    pub name: Option<String>,
    pub model_id: Option<Uuid>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub extra: Option<serde_json::Value>,
}

impl ParameterUpdateInput {
    pub fn merge(&self, model: &mut entity::parameter::ActiveModel) {
        model.updated_at = Set(Utc::now().naive_utc());

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
    }
}
