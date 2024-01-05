use std::collections::HashMap;

use async_graphql::InputObject;
use dojo_macros::UpdateModel;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(InputObject)]
pub struct ThreadCreateInput {
    pub name: String,
    pub slug: String,
}

#[derive(InputObject, UpdateModel)]
pub struct ThreadUpdateInput {
    pub name: Option<String>,
}

#[derive(Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThreadExecuteInput {
    pub thread_version_id: Uuid,
    pub parameter_id: Uuid,
    pub api_key_id: Uuid,
    pub variables: HashMap<String, String>,
}
