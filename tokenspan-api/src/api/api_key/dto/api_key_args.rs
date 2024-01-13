use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;
use uuid::Uuid;

#[derive(InputObject, Default)]
pub struct ApiKeyWhereProviderIdArgs {
    pub equals: Option<Uuid>,
}

#[derive(InputObject, Default)]
pub struct ApiKeyWhereArgs {
    pub provider_id: Option<ApiKeyWhereProviderIdArgs>,
}

#[derive(InputObject, Default)]
pub struct ApiKeyArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub r#where: Option<ApiKeyWhereArgs>,
}
