use sea_orm_migration::prelude::*;

use crate::m20231216_072112_create_task_version_table::TaskVersion;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Message::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Message::TaskVersionId).uuid().not_null())
                    .col(ColumnDef::new(Message::Raw).string())
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .col(ColumnDef::new(Message::Role).string().not_null())
                    .col(ColumnDef::new(Message::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Message::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-message-task-version-id")
                    .from(Message::Table, Message::TaskVersionId)
                    .to(TaskVersion::Table, TaskVersion::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    TaskVersionId,
    Raw,
    Content,
    Role,
    CreatedAt,
    UpdatedAt,
}
