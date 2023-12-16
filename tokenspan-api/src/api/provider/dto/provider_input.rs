use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ProviderCreateInput {
    pub name: String,
    pub slug: String,
}

#[derive(InputObject)]
pub struct ProviderUpdateInput {
    pub name: Option<String>,
    pub slug: Option<String>,
}
