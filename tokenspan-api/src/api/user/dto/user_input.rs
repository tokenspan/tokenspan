use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub email: String,
    pub username: String,
    pub password: String,
}
