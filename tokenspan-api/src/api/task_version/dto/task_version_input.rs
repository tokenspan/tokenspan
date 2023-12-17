use async_graphql::InputObject;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::dto::parameter_input::{ParameterCreateInput, ParameterMutationInput};
use crate::api::dto::MessageCreateInput;

#[derive(InputObject, TypedBuilder)]
pub struct TaskVersionCreateInput {
    pub task_id: Uuid,
    #[builder(default = "0.0.0".to_string())]
    pub semver: String,
    #[builder(default = 0)]
    pub version: u32,
    #[builder(default)]
    pub release_note: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub document: Option<String>,
    pub messages: Vec<MessageCreateInput>,
    pub parameters: Vec<ParameterCreateInput>,
}

#[derive(InputObject)]
pub struct TaskVersionUpdateInput {
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Option<Vec<MessageCreateInput>>,
    pub parameters: Option<Vec<ParameterMutationInput>>,
}

impl TaskVersionUpdateInput {
    pub fn copy(&self, model: &mut entity::task_version::ActiveModel) {
        if let Some(ref description) = self.description {
            model.description = Set(Some(description.clone()));
        }

        if let Some(ref document) = self.document {
            model.document = Set(Some(document.clone()));
        }

        model.updated_at = Set(Utc::now().naive_utc());
    }
}
