use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::predicates::*;
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::domains::function::dto::{FunctionArgs, FunctionCreateInput, FunctionUpdateInput};
use crate::domains::models::Function;

#[async_trait::async_trait]
pub trait FunctionServiceExt {
    async fn paginate(&self, args: FunctionArgs) -> Result<Pagination<Function>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Function>>;
    async fn find_by_slug(&self, slug: &String) -> Result<Option<Function>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Function>>;
    async fn create(&self, input: FunctionCreateInput, owner_id: Uuid) -> Result<Function>;
    async fn update_by_id(&self, id: &Uuid, input: FunctionUpdateInput) -> Result<Function>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<Function>;
}

pub type FunctionServiceDyn = Arc<dyn FunctionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct FunctionService {
    db: Database,
}

#[async_trait::async_trait]
impl FunctionServiceExt for FunctionService {
    async fn paginate(&self, args: FunctionArgs) -> Result<Pagination<Function>> {
        let mut predicates = vec![];
        if let Some(r#where) = &args.r#where {
            if let Some(name) = &r#where.name {
                predicates.push(text_search("name", "simple", name));
            }

            if let Some(description) = &r#where.description {
                predicates.push(text_search("name", "simple", description));
            }
        }

        self.db
            .bind::<Function>()
            .where_by(and(&predicates))
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Function>> {
        self.db
            .bind::<Function>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_slug(&self, slug: &String) -> Result<Option<Function>> {
        self.db
            .bind::<Function>()
            .where_by(equals("slug", slug))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Function>> {
        self.db
            .bind::<Function>()
            .where_by(in_list("id", &ids))
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

        self.db.insert(&input).exec().await
    }

    async fn update_by_id(&self, id: &Uuid, input: FunctionUpdateInput) -> Result<Function> {
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<Function> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<FunctionService> for FunctionServiceDyn {
    fn from(value: FunctionService) -> Self {
        Arc::new(value) as Self
    }
}
