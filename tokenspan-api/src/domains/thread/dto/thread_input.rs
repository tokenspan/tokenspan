use std::collections::HashMap;

use async_graphql::InputObject;
use dojo_macros::UpdateModel;
use serde::Deserialize;
use strum_macros::{Display, EnumString};
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
    pub slug: Option<String>,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, EnumString, Display)]
pub enum ToolType {
    #[strum(serialize = "function")]
    #[serde(rename = "function")]
    Function,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct ToolInput {
    #[serde(rename = "type")]
    pub ty: ToolType,
    pub id: Uuid,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct ThreadExecuteInput {
    pub thread_version_id: Uuid,
    pub parameter_id: Uuid,
    pub api_key_id: Uuid,
    #[serde(default)]
    pub tools: Vec<ToolInput>,
    pub variables: HashMap<String, String>,
}
