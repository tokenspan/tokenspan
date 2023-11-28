use async_graphql::InputObject;

#[derive(InputObject)]
pub struct SignUpInput {
    pub email: String,
    pub username: String,
    #[graphql(secret)]
    pub password: String,
}

#[derive(InputObject)]
pub struct SignInInput {
    pub email: String,
    #[graphql(secret)]
    pub password: String,
}

#[derive(InputObject)]
pub struct RefreshTokenInput {
    pub refresh_token: String,
}
