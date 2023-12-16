use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Task::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Task::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Task::Name).string().not_null())
                    .col(ColumnDef::new(Task::Slug).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Task::Private)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Task::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Task::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-task-owner-id")
                    .from(Task::Table, Task::OwnerId)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-task-slug")
                    .table(Task::Table)
                    .col(Task::Slug)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx-task-slug").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Task {
    Table,
    Id,
    OwnerId,
    Name,
    Slug,
    Private,
    CreatedAt,
    UpdatedAt,
}
