use async_graphql::InputObject;
use rabbit_macros::UpdateModel;
use uuid::Uuid;

#[derive(InputObject)]
pub struct ApiKeyCreateInput {
    pub name: String,
    #[graphql(secret)]
    pub key: String,
    pub provider_id: Uuid,
}

#[derive(InputObject, UpdateModel)]
pub struct ApiKeyUpdateInput {
    pub name: Option<String>,
}
