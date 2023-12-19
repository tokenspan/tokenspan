use async_graphql::InputObject;

#[derive(InputObject)]
pub struct UserCreateInput {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct UserUpdateInput {
    pub email: Option<String>,
    pub username: Option<String>,
}
