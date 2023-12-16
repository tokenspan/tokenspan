use crate::api::models::TaskId;
use async_graphql::InputObject;
use tokenspan_extra::pagination::Cursor;

#[derive(InputObject)]
pub struct ExecutionArgs {
    pub task_id: TaskId,
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
