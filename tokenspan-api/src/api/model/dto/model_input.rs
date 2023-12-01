use crate::api::models::ProviderId;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ModelCreateInput {
    pub name: String,
    pub description: String,
    pub context: u32,
    pub pricing: String,
    pub provider_id: ProviderId,
}

#[derive(InputObject)]
pub struct ModelUpdateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub context: Option<u32>,
    pub pricing: Option<String>,
}
