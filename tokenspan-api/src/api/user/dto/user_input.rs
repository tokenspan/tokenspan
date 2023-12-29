use async_graphql::InputObject;
use dojo_macros::UpdateModel;

#[derive(InputObject)]
pub struct UserCreateInput {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(InputObject, UpdateModel)]
pub struct UserUpdateInput {
    pub email: Option<String>,
    pub username: Option<String>,
}
