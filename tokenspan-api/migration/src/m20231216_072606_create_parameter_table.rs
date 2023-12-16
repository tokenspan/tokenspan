use sea_orm_migration::prelude::*;

use crate::m20231216_064026_create_model_table::Model;
use crate::m20231216_072112_create_task_version_table::TaskVersion;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Parameter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Parameter::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Parameter::ModelId).uuid().not_null())
                    .col(ColumnDef::new(Parameter::TaskVersionId).uuid().not_null())
                    .col(ColumnDef::new(Parameter::Name).string().not_null())
                    .col(ColumnDef::new(Parameter::Temperature).float().not_null())
                    .col(ColumnDef::new(Parameter::MaxTokens).integer().not_null())
                    .col(
                        ColumnDef::new(Parameter::StopSequences)
                            .array(ColumnType::String(None))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Parameter::TopP).float().not_null())
                    .col(
                        ColumnDef::new(Parameter::FrequencyPenalty)
                            .float()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Parameter::PresencePenalty)
                            .float()
                            .default(0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Parameter::Extra).json())
                    .col(ColumnDef::new(Parameter::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Parameter::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-parameter-model-id")
                    .from(Parameter::Table, Parameter::ModelId)
                    .to(Model::Table, Model::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-parameter-task-version-id")
                    .from(Parameter::Table, Parameter::TaskVersionId)
                    .to(TaskVersion::Table, TaskVersion::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Parameter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Parameter {
    Table,
    Id,
    ModelId,
    TaskVersionId,
    Name,
    Temperature,
    MaxTokens,
    StopSequences,
    TopP,
    FrequencyPenalty,
    PresencePenalty,
    Extra,
    CreatedAt,
    UpdatedAt,
}
