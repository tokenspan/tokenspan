use std::sync::Arc;

use crate::api::dto::{MessageArgs, MessageCreateInput, MessageUpdateInput};
use crate::api::models::Message;
use anyhow::Result;
use axum::extract::FromRef;
use chrono::Utc;
use dojo_orm::ops::{and, eq, in_list};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::state::AppState;

#[async_trait::async_trait]
pub trait MessageServiceExt {
    async fn paginate(&self, args: MessageArgs) -> Result<Pagination<Cursor, Message>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>>;
    async fn find_by_thread_version_id(&self, thread_version_id: Uuid) -> Result<Vec<Message>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Message>>;
    async fn create(&self, input: MessageCreateInput, owner_id: Uuid) -> Result<Message>;
    async fn update_by_id(&self, id: Uuid, input: MessageUpdateInput) -> Result<Option<Message>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Message>>;
}

pub type MessageServiceDyn = Arc<dyn MessageServiceExt + Send + Sync>;

impl FromRef<AppState> for MessageServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.message_service.clone()
    }
}

#[derive(TypedBuilder)]
pub struct MessageService {
    db: Database,
}

#[async_trait::async_trait]
impl MessageServiceExt for MessageService {
    async fn paginate(&self, args: MessageArgs) -> Result<Pagination<Cursor, Message>> {
        self.db
            .bind::<Message>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>> {
        self.db
            .bind::<Message>()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn find_by_thread_version_id(&self, thread_version_id: Uuid) -> Result<Vec<Message>> {
        self.db
            .bind::<Message>()
            .where_by(and(&[eq("thread_version_id", &thread_version_id)]))
            .all()
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Message>> {
        self.db
            .bind::<Message>()
            .where_by(and(&[in_list("id", &ids)]))
            .all()
            .await
    }

    async fn create(&self, input: MessageCreateInput, owner_id: Uuid) -> Result<Message> {
        let input = Message {
            id: Uuid::new_v4(),
            role: input.role,
            thread_version_id: input.thread_version_id,
            owner_id,
            raw: input.raw,
            content: input.content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.db.insert(&input).await
    }

    async fn update_by_id(&self, id: Uuid, input: MessageUpdateInput) -> Result<Option<Message>> {
        self.db
            .update(&input)
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Message>> {
        self.db
            .delete()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }
}

impl From<MessageService> for MessageServiceDyn {
    fn from(value: MessageService) -> Self {
        Arc::new(value) as Self
    }
}