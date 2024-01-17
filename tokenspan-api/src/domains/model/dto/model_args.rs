use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;
use uuid::Uuid;

#[derive(InputObject, Default)]
pub struct ModelWhereProviderIdArgs {
    pub equals: Option<Uuid>,
}

#[derive(InputObject, Default)]
pub struct ModelWhereArgs {
    pub provider_id: Option<ModelWhereProviderIdArgs>,
}

#[derive(InputObject, Default)]
pub struct ModelArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub r#where: Option<ModelWhereArgs>,
}
