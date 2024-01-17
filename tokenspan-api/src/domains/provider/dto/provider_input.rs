use async_graphql::InputObject;
use dojo_macros::UpdateModel;

#[derive(InputObject)]
pub struct ProviderCreateInput {
    pub name: String,
    pub slug: String,
    pub base_url: String,
}

#[derive(InputObject, UpdateModel)]
pub struct ProviderUpdateInput {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub base_url: Option<String>,
}
