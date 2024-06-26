use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct UserArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub after: Option<Cursor>,
    pub before: Option<Cursor>,
}
