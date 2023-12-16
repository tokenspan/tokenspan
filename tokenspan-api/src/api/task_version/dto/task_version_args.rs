use async_graphql::{InputObject, OneofObject};

use tokenspan_extra::pagination::Cursor;

use crate::api::models::{TaskId, TaskVersionId};

#[derive(InputObject)]
pub struct TaskVersionArgs {
    pub task_id: TaskId,
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}

#[derive(InputObject)]
pub struct TaskVersionByVersion {
    pub task_id: TaskId,
    pub version: String,
}

#[derive(InputObject)]
pub struct TaskVersionByLatest {
    pub task_id: TaskId,
}

#[derive(OneofObject)]
pub enum TaskVersionBy {
    Id(TaskVersionId),
    Version(TaskVersionByVersion),
    Latest(TaskVersionByLatest),
}
