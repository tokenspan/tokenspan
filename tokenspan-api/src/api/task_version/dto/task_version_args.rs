use async_graphql::{InputObject, OneofObject};
use uuid::Uuid;

use tokenspan_extra::pagination::Cursor;

#[derive(InputObject)]
pub struct TaskVersionArgs {
    pub task_id: Uuid,
    pub take: Option<u64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}

#[derive(InputObject)]
pub struct TaskVersionByVersion {
    pub task_id: Uuid,
    pub version: String,
}

#[derive(InputObject)]
pub struct TaskVersionByLatest {
    pub task_id: Uuid,
}

#[derive(OneofObject)]
pub enum TaskVersionBy {
    Id(Uuid),
    Version(TaskVersionByVersion),
    Latest(TaskVersionByLatest),
}
