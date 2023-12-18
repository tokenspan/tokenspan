use sea_orm_migration::prelude::*;

use crate::extension::postgres::Type;
use crate::sea_orm::EnumIter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TaskVersionStatus::Table)
                    .values([TaskVersionStatus::Draft, TaskVersionStatus::Released])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TaskVersion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskVersion::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TaskVersion::TaskId).uuid().not_null())
                    .col(ColumnDef::new(TaskVersion::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(TaskVersion::Semver).string().not_null())
                    .col(ColumnDef::new(TaskVersion::Version).integer().not_null())
                    .col(ColumnDef::new(TaskVersion::Description).string())
                    .col(ColumnDef::new(TaskVersion::ReleaseNote).string())
                    .col(ColumnDef::new(TaskVersion::Document).string())
                    .col(
                        ColumnDef::new(TaskVersion::Status)
                            .enumeration(
                                TaskVersionStatus::Table,
                                [TaskVersionStatus::Draft, TaskVersionStatus::Released],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(TaskVersion::Messages).json().not_null())
                    .col(ColumnDef::new(TaskVersion::ReleasedAt).timestamp())
                    .col(
                        ColumnDef::new(TaskVersion::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TaskVersion::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-task-version-task-id")
                    .table(TaskVersion::Table)
                    .col(TaskVersion::TaskId)
                    .col(TaskVersion::Version)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-task-semver-task-id")
                    .table(TaskVersion::Table)
                    .col(TaskVersion::TaskId)
                    .col(TaskVersion::Semver)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-task-semver-task-id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-task-version-task-id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(TaskVersion::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(TaskVersionStatus::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum TaskVersion {
    Table,
    Id,
    TaskId,
    OwnerId,
    Semver,
    Version,
    Description,
    ReleaseNote,
    Document,
    Status,
    Messages,
    ReleasedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
enum TaskVersionStatus {
    Table,
    #[iden = "DRAFT"]
    Draft,
    #[iden = "RELEASED"]
    Released,
}
