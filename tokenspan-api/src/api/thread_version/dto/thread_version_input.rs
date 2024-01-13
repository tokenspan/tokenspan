use crate::api::models::ThreadVersionStatus;
use async_graphql::InputObject;
use dojo_macros::UpdateModel;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(InputObject, TypedBuilder)]
pub struct ThreadVersionCreateInput {
    pub thread_id: Uuid,
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
}

#[derive(InputObject, TypedBuilder)]
pub struct ThreadVersionPublishInput {
    pub semver: String,
    pub release_note: String,
}

#[derive(InputObject, UpdateModel, Debug, Default)]
pub struct ThreadVersionUpdateInput {
    pub description: Option<String>,
    pub document: Option<String>,
    #[graphql(skip, default)]
    pub release_note: Option<String>,
    #[graphql(skip, default)]
    pub status: Option<ThreadVersionStatus>,
}
