use crate::api::models::ProviderId;
use crate::prisma::model;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateModelInput {
    pub name: String,
    pub description: String,
    pub context: i32,
    pub pricing: String,
    pub provider_id: ProviderId,
}

#[derive(InputObject)]
pub struct UpdateModelInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub context: Option<i32>,
    pub pricing: Option<String>,
}

impl From<UpdateModelInput> for Vec<model::SetParam> {
    fn from(value: UpdateModelInput) -> Self {
        let mut params = Vec::new();
        if let Some(name) = value.name {
            params.push(model::name::set(name));
        }

        if let Some(description) = value.description {
            params.push(model::description::set(description));
        }

        if let Some(context) = value.context {
            params.push(model::context::set(context));
        }

        if let Some(pricing) = value.pricing {
            params.push(model::pricing::set(pricing));
        }

        params
    }
}
