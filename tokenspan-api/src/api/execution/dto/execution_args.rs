use async_graphql::InputObject;
use uuid::Uuid;

use dojo_orm::pagination::Cursor;

#[derive(InputObject)]
pub struct ExecutionArgs {
    pub task_id: Uuid,
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
