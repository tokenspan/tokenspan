use crate::prisma::view;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateViewInput {
    pub name: String,
    pub config: Option<serde_json::Value>,
}

#[derive(InputObject)]
pub struct UpdateViewInput {
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
}

impl From<UpdateViewInput> for Vec<view::SetParam> {
    fn from(value: UpdateViewInput) -> Self {
        let mut params = Vec::new();
        if let Some(name) = value.name {
            params.push(view::name::set(name));
        }

        params.push(view::config::set(value.config));

        params
    }
}
