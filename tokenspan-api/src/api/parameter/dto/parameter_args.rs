use async_graphql::InputObject;
use uuid::Uuid;

use tokenspan_extra::pagination::Cursor;

#[derive(InputObject)]
pub struct ParameterArgs {
    pub task_id: Uuid,
    pub take: Option<u64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
