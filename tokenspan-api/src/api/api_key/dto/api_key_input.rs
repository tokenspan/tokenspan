use async_graphql::InputObject;
use uuid::Uuid;

#[derive(InputObject)]
pub struct ApiKeyCreateInput {
    pub name: String,
    #[graphql(secret)]
    pub key: String,
    pub provider_id: Uuid,
}

#[derive(InputObject)]
pub struct ApiKeyUpdateInput {
    pub name: Option<String>,
}
