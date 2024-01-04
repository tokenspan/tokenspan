use async_graphql::InputObject;
use dojo_macros::UpdateModel;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::dto::MessageCreateInput;

#[derive(InputObject, TypedBuilder)]
pub struct TaskVersionCreateInput {
    pub task_id: Uuid,
    #[builder(default = "0.0.0".to_string())]
    pub semver: String,
    #[builder(default = 0)]
    pub version: i32,
    #[builder(default)]
    pub release_note: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub document: Option<String>,
    pub messages: Vec<MessageCreateInput>,
}

#[derive(InputObject, UpdateModel, Debug)]
pub struct TaskVersionUpdateInput {
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Option<Vec<MessageCreateInput>>,
}
