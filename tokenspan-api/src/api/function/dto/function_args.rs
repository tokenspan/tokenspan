use async_graphql::InputObject;
use dojo_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct FunctionArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
