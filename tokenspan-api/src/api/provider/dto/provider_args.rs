use async_graphql::InputObject;
use rabbit_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct ProviderArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
