use crate::prisma::provider;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateProviderInput {
    pub name: String,
}

#[derive(InputObject)]
pub struct UpdateProviderInput {
    pub name: Option<String>,
}

impl From<UpdateProviderInput> for Vec<provider::SetParam> {
    fn from(value: UpdateProviderInput) -> Self {
        let mut params = Vec::new();
        if let Some(name) = value.name {
            params.push(provider::name::set(name));
        }

        params
    }
}
