use async_graphql::InputObject;
use tokenspan_extra::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct TaskArgs {
    pub take: Option<u64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
