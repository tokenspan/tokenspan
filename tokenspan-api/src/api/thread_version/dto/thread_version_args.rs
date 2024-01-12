use async_graphql::{InputObject, OneofObject};
use uuid::Uuid;

use dojo_orm::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct ThreadVersionArgs {
    pub thread_id: Uuid,
    pub first: Option<i64>,
    pub last: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
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
