use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct UserArgs {
    pub take: Option<i64>,
    pub after: Option<Cursor>,
    pub before: Option<Cursor>,
}
