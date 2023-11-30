use crate::prisma::view;
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

impl From<ViewUpdateInput> for Vec<view::SetParam> {
    fn from(value: ViewUpdateInput) -> Self {
        let mut params = Vec::new();
        if let Some(name) = value.name {
            params.push(view::name::set(name));
        }

        params.push(view::config::set(value.config));

        params
    }
}
