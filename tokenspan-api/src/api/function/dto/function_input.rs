use async_graphql::InputObject;
use dojo_macros::UpdateModel;

#[derive(InputObject)]
pub struct FunctionCreateInput {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub response: Option<serde_json::Value>,
}

#[derive(InputObject, UpdateModel)]
pub struct FunctionUpdateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parameters: Option<serde_json::Value>,
    #[dojo(nullable)]
    pub response: Option<serde_json::Value>,
}
