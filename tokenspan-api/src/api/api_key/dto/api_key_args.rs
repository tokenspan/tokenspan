use crate::repository::PaginateArgs;
use async_graphql::InputObject;
use tokenspan_extra::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct ApiKeyArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}

impl From<ApiKeyArgs> for PaginateArgs {
    fn from(args: ApiKeyArgs) -> Self {
        Self {
            take: args.take,
            before: args.before,
            after: args.after,
        }
    }
}
