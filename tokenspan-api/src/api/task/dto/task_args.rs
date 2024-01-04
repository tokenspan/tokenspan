use async_graphql::InputObject;
use chrono::NaiveDateTime;
use dojo_orm::pagination::Cursor;
use uuid::Uuid;

#[derive(InputObject, Default)]
pub struct TaskSort {
    pub created_at: Option<NaiveDateTime>,
}

#[derive(InputObject, Default)]
pub struct TaskFilter {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub owner_id: Option<Uuid>,
}

#[derive(InputObject, Default)]
pub struct TaskArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub filter: Option<TaskFilter>,
    pub sort: Option<TaskSort>,
}
