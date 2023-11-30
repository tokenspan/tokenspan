use crate::api::repositories::*;
use mongodb::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Clone)]
pub struct Repository<T> {
    pub collection_name: &'static str,
    pub collection: mongodb::Collection<T>,
    pub db: mongodb::Database,
}

impl<T> Repository<T> {
    pub fn new(db: mongodb::Database, collection_name: &'static str) -> Self {
        let collection = db.collection(collection_name);
        Self {
            collection_name,
            collection,
            db,
        }
    }
}

pub struct RootRepository {
    pub user: Repository<UserDoc>,
    pub view: Repository<ViewDoc>,
}

impl RootRepository {
    pub async fn new_with_uri(uri: String) -> Self {
        let client = Client::with_uri_str(uri).await.unwrap();
        let database = client.database("tokenspan");
        let user = Repository::new(database.clone(), "users");
        let view = Repository::new(database.clone(), "views");
        Self { user, view }
    }
}
