use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use futures_util::future::try_join_all;
use futures_util::StreamExt;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter, TransactionTrait,
};
use uuid::Uuid;

use crate::api::dto::{MessageCreateInput, MessageUpdateInput};
use crate::api::message::message_error::MessageError;
use crate::api::models::Message;

#[async_trait::async_trait]
pub trait MessageServiceExt {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>>;
    async fn find_by_task_version_id(&self, id: Uuid) -> Result<Vec<Message>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Message>>;
    async fn create(&self, input: &[MessageCreateInput]) -> Result<Vec<Message>>;
    async fn update_by_id(&self, id: Uuid, input: MessageUpdateInput) -> Result<Message>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Message>;
}

pub type MessageServiceDyn = Arc<dyn MessageServiceExt + Send + Sync>;

pub struct MessageService {
    db: DatabaseConnection,
}

impl MessageService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl MessageServiceExt for MessageService {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>> {
        let message = entity::message::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))?
            .map(|message| message.into());

        Ok(message)
    }

    async fn find_by_task_version_id(&self, id: Uuid) -> Result<Vec<Message>> {
        let message = entity::message::Entity::find()
            .filter(entity::message::Column::TaskVersionId.eq(id))
            .all(&self.db)
            .await
            .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|message| message.into())
            .collect();

        Ok(message)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Message>> {
        let messages = entity::message::Entity::find()
            .filter(entity::message::Column::Id.is_in(ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|message| message.into())
            .collect();

        Ok(messages)
    }

    async fn create(&self, input: &[MessageCreateInput]) -> Result<Vec<Message>> {
        let tx = self.db.begin().await?;

        let mut futs = vec![];
        for input in input {
            let fut = async {
                entity::message::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    task_version_id: Set(input.task_version_id),
                    raw: Set(input.raw.clone()),
                    content: Set(input.content.clone()),
                    role: Set(input.role.clone()),
                    created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                }
                .insert(&tx)
                .await
                .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))
            };
            futs.push(fut);
        }

        let messages = try_join_all(futs)
            .await?
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<_>>();

        tx.commit().await?;

        Ok(messages)
    }

    async fn update_by_id(&self, id: Uuid, input: MessageUpdateInput) -> Result<Message> {
        let mut updated_message = entity::message::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(MessageError::Unknown(anyhow::anyhow!("Message not found")))?
            .into_active_model();

        input.copy(&mut updated_message);

        let updated_message = updated_message
            .update(&self.db)
            .await
            .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))?
            .into();

        Ok(updated_message)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Message> {
        let deleted_message = entity::message::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(MessageError::Unknown(anyhow::anyhow!("Message not found")))?;

        deleted_message
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| MessageError::Unknown(anyhow::anyhow!(e)))?;

        Ok(deleted_message.into())
    }
}

impl From<MessageService> for MessageServiceDyn {
    fn from(value: MessageService) -> Self {
        Arc::new(value) as Self
    }
}
