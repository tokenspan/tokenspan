use std::sync::Arc;

use anyhow::Result;
use axum::extract::FromRef;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::prelude::*;
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::domains::dto::{MessageArgs, MessageCreateInput, MessageUpdateInput};
use crate::domains::models::Message;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait MessageServiceExt {
    async fn paginate(&self, args: MessageArgs) -> Result<Pagination<Message>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Message>>;
    async fn find_by_thread_version_id(&self, thread_version_id: &Uuid) -> Result<Vec<Message>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Message>>;
    async fn create(&self, input: MessageCreateInput, owner_id: Uuid) -> Result<Message>;
    async fn duplicate_by_thread_version_id(
        &self,
        current_thread_version_id: &Uuid,
        new_thread_version_id: Uuid,
    ) -> Result<Vec<Message>>;
    async fn update_by_id(&self, id: &Uuid, input: MessageUpdateInput) -> Result<Message>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<Message>;
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
    async fn paginate(&self, args: MessageArgs) -> Result<Pagination<Message>> {
        self.db
            .bind::<Message>()
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Message>> {
        self.db
            .bind::<Message>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_thread_version_id(&self, thread_version_id: &Uuid) -> Result<Vec<Message>> {
        self.db
            .bind::<Message>()
            .where_by(equals("thread_version_id", thread_version_id))
            .limit(20)
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Message>> {
        self.db
            .bind::<Message>()
            .where_by(in_list("id", &ids))
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
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).await
    }

    async fn duplicate_by_thread_version_id(
        &self,
        current_thread_version_id: &Uuid,
        new_thread_version_id: Uuid,
    ) -> Result<Vec<Message>> {
        let mut messages = self
            .find_by_thread_version_id(current_thread_version_id)
            .await?;

        if messages.is_empty() {
            return Ok(vec![]);
        }

        for message in &mut messages {
            message.id = Uuid::new_v4();
            message.thread_version_id = new_thread_version_id;
            message.created_at = Utc::now().naive_utc();
            message.updated_at = Utc::now().naive_utc();
        }

        self.db.insert_many(&messages).await
    }

    async fn update_by_id(&self, id: &Uuid, input: MessageUpdateInput) -> Result<Message> {
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<Message> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<MessageService> for MessageServiceDyn {
    fn from(value: MessageService) -> Self {
        Arc::new(value) as Self
    }
}
