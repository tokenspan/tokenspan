use crate::m20220101_000001_create_user_table::User;
use crate::m20231216_070852_create_task_table::Task;
use crate::m20231216_072112_create_task_version_table::TaskVersion;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Execution::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Execution::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Execution::TaskId).uuid().not_null())
                    .col(ColumnDef::new(Execution::TaskVersionId).uuid().not_null())
                    .col(ColumnDef::new(Execution::ExecutorId).uuid().not_null())
                    .col(ColumnDef::new(Execution::Elapsed).json().not_null())
                    .col(
                        ColumnDef::new(Execution::Messages)
                            .array(ColumnType::Json)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Execution::Parameter).json().not_null())
                    .col(ColumnDef::new(Execution::Output).json())
                    .col(ColumnDef::new(Execution::Error).json())
                    .col(ColumnDef::new(Execution::Usage).json())
                    .col(ColumnDef::new(Execution::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Execution::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-execution-task-id")
                    .from(Execution::Table, Execution::TaskId)
                    .to(Task::Table, Task::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-execution-task-version-id")
                    .from(Execution::Table, Execution::TaskVersionId)
                    .to(TaskVersion::Table, TaskVersion::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-execution-executor-id")
                    .from(Execution::Table, Execution::ExecutorId)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Execution::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Execution {
    Table,
    Id,
    TaskId,
    TaskVersionId,
    ExecutorId,
    Elapsed,
    Messages,
    Parameter,
    Output,
    Error,
    Usage,
    CreatedAt,
    UpdatedAt,
}
