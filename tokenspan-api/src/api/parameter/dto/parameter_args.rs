use async_graphql::InputObject;
use uuid::Uuid;

use dojo_orm::pagination::Cursor;

#[derive(InputObject)]
pub struct ParameterArgs {
    pub thread_id: Uuid,
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
