use async_graphql::InputObject;
use tokenspan_extra::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct UserArgs {
    pub take: Option<u64>,
    pub after: Option<Cursor>,
    pub before: Option<Cursor>,
}
