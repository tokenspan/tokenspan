use async_graphql::InputObject;
use uuid::Uuid;

use dojo_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct ExecutionWhereThreadIdArgs {
    pub equals: Option<Uuid>,
}

#[derive(InputObject, Default)]
pub struct ExecutionWhereArgs {
    pub thread_id: Option<ExecutionWhereThreadIdArgs>,
}

#[derive(InputObject, Default)]
pub struct ExecutionArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub r#where: Option<ExecutionWhereArgs>,
}
