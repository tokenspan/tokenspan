use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ProviderCreateInput {
    pub name: String,
}

#[derive(InputObject)]
pub struct ProviderUpdateInput {
    pub name: Option<String>,
}
