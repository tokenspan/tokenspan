use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct FunctionArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
