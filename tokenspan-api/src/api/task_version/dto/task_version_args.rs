use async_graphql::InputObject;

use tokenspan_utils::pagination::Cursor;

use crate::api::models::TaskId;
use crate::repository::PaginateArgs;

#[derive(InputObject)]
pub struct TaskVersionArgs {
    pub task_id: TaskId,
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}

impl From<TaskVersionArgs> for PaginateArgs {
    fn from(args: TaskVersionArgs) -> Self {
        Self {
            take: args.take,
            before: args.before,
            after: args.after,
        }
    }
}
