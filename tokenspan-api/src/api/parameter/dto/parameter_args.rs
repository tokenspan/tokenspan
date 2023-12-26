use async_graphql::InputObject;
use uuid::Uuid;

use rabbit_orm::pagination::Cursor;

#[derive(InputObject)]
pub struct ParameterArgs {
    pub task_id: Uuid,
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}
