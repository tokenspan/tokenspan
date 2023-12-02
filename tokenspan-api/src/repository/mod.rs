use bson::doc;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::Client;
use serde::de::DeserializeOwned;

use tokenspan_utils::pagination::{Cursor, CursorExt, Pagination};

use crate::api::repositories::*;

#[derive(Clone)]
pub struct Repository<TData> {
    pub collection_name: &'static str,
    pub collection: mongodb::Collection<TData>,
    pub db: mongodb::Database,
}

impl<TData> Repository<TData>
where
    TData: DeserializeOwned + Sync + Send + Unpin,
{
    pub fn new(db: mongodb::Database, collection_name: &'static str) -> Self {
        let collection = db.collection(collection_name);
        Self {
            collection_name,
            collection,
            db,
        }
    }

    pub async fn paginate<TNode: CursorExt<Cursor> + From<TData>>(
        &self,
        take: Option<i64>,
        before: Option<Cursor>,
        after: Option<Cursor>,
    ) -> mongodb::error::Result<Pagination<Cursor, TNode>> {
        let take = take.unwrap_or(1);
        let limit = take
            + if after.is_some() || before.is_some() {
                2
            } else {
                1
            };

        let filter = after
            .clone()
            .map(|cursor| doc! { "_id": { "$lte": ObjectId::parse_str(cursor.id).unwrap() } })
            .or_else(|| {
                before.clone().map(
                    |cursor| doc! { "_id": { "$gte": ObjectId::parse_str(cursor.id).unwrap() } },
                )
            })
            .unwrap_or(doc! {});

        let options = FindOptions::builder()
            .sort(doc! {
                "_id": -1
            })
            .limit(limit)
            .build();

        let items = self
            .collection
            .find(filter, Some(options))
            .await?
            .try_collect::<Vec<TData>>()
            .await?
            .into_iter()
            .map(|doc| doc.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, before, after, take))
    }

    pub async fn find_by_id<T>(&self, id: T) -> mongodb::error::Result<Option<TData>>
    where
        T: Send + Sync,
        ObjectId: From<T>,
    {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };

        self.collection.find_one(filter, None).await
    }

    pub async fn find_many_by_ids<T>(&self, ids: Vec<T>) -> mongodb::error::Result<Vec<TData>>
    where
        T: Send + Sync,
        ObjectId: From<T>,
    {
        let ids = ids
            .into_iter()
            .map(|id| id.into())
            .collect::<Vec<ObjectId>>();

        let cursor = self
            .collection
            .find(
                doc! {
                    "_id": {
                        "$in": ids
                    }
                },
                None,
            )
            .await?;

        cursor.try_collect().await
    }

    pub async fn delete_by_id<T>(&self, id: T) -> mongodb::error::Result<Option<TData>>
    where
        T: Send + Sync,
        ObjectId: From<T>,
    {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };

        self.collection.find_one_and_delete(filter, None).await
    }

    pub async fn count(&self) -> mongodb::error::Result<u64> {
        let filter = doc! {};

        self.collection.count_documents(filter, None).await
    }
}

pub struct RootRepository {
    pub user: Repository<UserEntity>,
    pub view: Repository<ViewEntity>,
    pub api_key: Repository<ApiKeyEntity>,
    pub task_version: Repository<TaskVersionEntity>,
    pub task: Repository<TaskEntity>,
    pub provider: Repository<ProviderEntity>,
    pub parameter: Repository<ParameterEntity>,
    pub model: Repository<ModelEntity>,
    pub execution: Repository<ExecutionEntity>,
}

impl RootRepository {
    pub async fn new_with_uri(uri: String) -> Self {
        let client = Client::with_uri_str(uri).await.unwrap();
        let database = client.database("tokenspan");
        let user = Repository::new(database.clone(), "users");
        let view = Repository::new(database.clone(), "views");
        let api_key = Repository::new(database.clone(), "api_keys");
        let task_version = Repository::new(database.clone(), "task_versions");
        let task = Repository::new(database.clone(), "tasks");
        let provider = Repository::new(database.clone(), "providers");
        let parameter = Repository::new(database.clone(), "parameters");
        let model = Repository::new(database.clone(), "models");
        let execution = Repository::new(database.clone(), "executions");

        Self {
            user,
            view,
            api_key,
            task_version,
            task,
            provider,
            parameter,
            model,
            execution,
        }
    }
}
