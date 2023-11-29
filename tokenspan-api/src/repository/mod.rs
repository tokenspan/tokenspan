use crate::api::repositories::*;
use mongodb::Client;

#[derive(Clone)]
pub struct Repository {
    pub user: UserRepository,
}

impl Repository {
    pub async fn new_with_uri(uri: String) -> Self {
        let client = Client::with_uri_str(uri).await.unwrap();
        let database = client.database("tokenspan");
        let user = UserRepository::new(database);
        Self { user }
    }
}
