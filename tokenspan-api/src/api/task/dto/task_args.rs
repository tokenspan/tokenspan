use async_graphql::InputObject;
use tokenspan_utils::pagination::Cursor;

#[derive(InputObject)]
pub struct TaskArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
