use crate::api::models::ProviderId;
use crate::prisma::api_key;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateApiKeyInput {
    pub name: String,
    #[graphql(secret)]
    pub key: String,
    pub provider_id: ProviderId,
}

#[derive(InputObject)]
pub struct UpdateApiKeyInput {
    pub name: Option<String>,
}

impl From<UpdateApiKeyInput> for Vec<api_key::SetParam> {
    fn from(value: UpdateApiKeyInput) -> Self {
        let mut params = Vec::new();
        if let Some(name) = value.name {
            params.push(api_key::name::set(name));
        }

        params
    }
}
