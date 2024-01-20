use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;
use uuid::Uuid;

#[derive(InputObject, Default)]
pub struct ThreadWhereArgs {
    pub owner_id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(InputObject, Default)]
pub struct ThreadArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub r#where: Option<ThreadWhereArgs>,
}
