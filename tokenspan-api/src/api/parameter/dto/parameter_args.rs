use crate::repository::PaginateArgs;
use async_graphql::InputObject;
use tokenspan_utils::pagination::Cursor;

#[derive(InputObject, Default)]
pub struct ParameterArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}

impl From<ParameterArgs> for PaginateArgs {
    fn from(args: ParameterArgs) -> Self {
        Self {
            take: args.take,
            before: args.before,
            after: args.after,
        }
    }
}
