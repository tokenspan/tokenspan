use crate::repository::PaginateArgs;
use async_graphql::InputObject;
use tokenspan_extra::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct ProviderArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}

impl From<ProviderArgs> for PaginateArgs {
    fn from(args: ProviderArgs) -> Self {
        Self {
            take: args.take,
            before: args.before,
            after: args.after,
        }
    }
}
