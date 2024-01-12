use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;
use uuid::Uuid;

#[derive(InputObject, Default)]
pub struct ModelWhereProviderIdInput {
    pub equals: Option<Uuid>,
}

#[derive(InputObject, Default)]
pub struct ModelWhereInput {
    pub provider_id: Option<ModelWhereProviderIdInput>,
}

#[derive(InputObject, Default)]
pub struct ModelArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub r#where: Option<ModelWhereInput>,
}
