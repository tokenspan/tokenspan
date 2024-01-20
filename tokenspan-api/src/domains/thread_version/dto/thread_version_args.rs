use async_graphql::{InputObject, OneofObject};
use uuid::Uuid;

use dojo_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct ThreadVersionWhereArgs {
    pub thread_id: Option<Uuid>,
    pub description: Option<String>,
    pub document: Option<String>,
}

#[derive(InputObject, Default)]
pub struct ThreadVersionArgs {
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub r#where: Option<ThreadVersionWhereArgs>,
}

#[derive(InputObject)]
pub struct ThreadVersionBySemver {
    pub thread_id: Uuid,
    pub semver: String,
}

#[derive(InputObject)]
pub struct ThreadVersionByLatest {
    pub thread_id: Uuid,
}

#[derive(OneofObject)]
pub enum ThreadVersionBy {
    Id(Uuid),
    Semver(ThreadVersionBySemver),
    Latest(ThreadVersionByLatest),
}
