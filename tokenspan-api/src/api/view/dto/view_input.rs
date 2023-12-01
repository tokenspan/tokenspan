use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ViewCreateInput {
    pub name: String,
    pub config: Option<serde_json::Value>,
}

#[derive(InputObject)]
pub struct ViewUpdateInput {
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
}
