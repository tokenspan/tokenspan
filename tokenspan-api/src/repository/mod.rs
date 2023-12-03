use bson::oid::ObjectId;
use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::Client;
use serde::de::DeserializeOwned;

use tokenspan_utils::pagination::{Cursor, CursorExt, Pagination};

use crate::api::repositories::*;

pub struct PaginateArgs {
    pub take: Option<i64>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
}

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
        args: PaginateArgs,
    ) -> mongodb::error::Result<Pagination<Cursor, TNode>> {
        self.paginate_with_filter(doc! {}, args).await
    }

    pub async fn paginate_with_filter<TNode: CursorExt<Cursor> + From<TData>>(
        &self,
        filter: Document,
        args: PaginateArgs,
    ) -> mongodb::error::Result<Pagination<Cursor, TNode>> {
        let take = args.take.unwrap_or(1);
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };

        let mut default_filter = args
            .after
            .clone()
            .map(|cursor| doc! { "_id": { "$lte": ObjectId::parse_str(cursor.id).unwrap() } })
            .or_else(|| {
                args.before.clone().map(
                    |cursor| doc! { "_id": { "$gte": ObjectId::parse_str(cursor.id).unwrap() } },
                )
            })
            .unwrap_or_default();

        default_filter.extend(filter);

        let options = FindOptions::builder()
            .sort(doc! {
                "_id": -1
            })
            .limit(limit)
            .build();

        let count_fut = self
            .collection
            .count_documents(default_filter.clone(), None);

        let find_fut = self.collection.find(default_filter, Some(options));

        let (count, cursor) =
            tokio::try_join!(count_fut, find_fut).map_err(mongodb::error::Error::custom)?;
        let items = cursor
            .try_collect::<Vec<TData>>()
            .await?
            .into_iter()
            .map(|doc| doc.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
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
