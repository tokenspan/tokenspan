use crate::api::models::ProviderId;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ApiKeyCreateInput {
    pub name: String,
    #[graphql(secret)]
    pub key: String,
    pub provider_id: ProviderId,
}

#[derive(InputObject)]
pub struct ApiKeyUpdateInput {
    pub name: Option<String>,
}
