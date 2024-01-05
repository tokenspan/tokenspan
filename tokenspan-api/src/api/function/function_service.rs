use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::ops::{and, eq, in_list};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::function::dto::{FunctionArgs, FunctionCreateInput, FunctionUpdateInput};
use crate::api::models::Function;

#[async_trait::async_trait]
pub trait FunctionServiceExt {
    async fn paginate(&self, args: FunctionArgs) -> Result<Pagination<Cursor, Function>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Function>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Function>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Function>>;
    async fn create(&self, input: FunctionCreateInput, owner_id: Uuid) -> Result<Function>;
    async fn update_by_id(&self, id: Uuid, input: FunctionUpdateInput) -> Result<Option<Function>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Function>>;
}

pub type FunctionServiceDyn = Arc<dyn FunctionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct FunctionService {
    db: Database,
}

#[async_trait::async_trait]
impl FunctionServiceExt for FunctionService {
    async fn paginate(&self, args: FunctionArgs) -> Result<Pagination<Cursor, Function>> {
        self.db
            .bind::<Function>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Function>> {
        self.db
            .bind::<Function>()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Function>> {
        self.db
            .bind::<Function>()
            .where_by(and(&[eq("slug", &slug)]))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Function>> {
        self.db
            .bind::<Function>()
            .where_by(and(&[in_list("id", &ids)]))
            .all()
            .await
    }

    async fn create(&self, input: FunctionCreateInput, owner_id: Uuid) -> Result<Function> {
        let input = Function {
            id: Uuid::new_v4(),
            name: input.name,
            description: input.description,
            parameters: input.parameters,
            response: input.response,
            owner_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).await
    }

    async fn update_by_id(&self, id: Uuid, input: FunctionUpdateInput) -> Result<Option<Function>> {
        self.db
            .update(&input)
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Function>> {
        self.db
            .delete()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }
}

impl From<FunctionService> for FunctionServiceDyn {
    fn from(value: FunctionService) -> Self {
        Arc::new(value) as Self
    }
}
