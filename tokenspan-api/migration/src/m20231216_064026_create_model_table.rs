use sea_orm_migration::prelude::*;

use crate::m20231216_063415_create_provider_table::Provider;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Model::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Model::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Model::ProviderId).uuid().not_null())
                    .col(ColumnDef::new(Model::Name).string().not_null())
                    .col(ColumnDef::new(Model::Description).string().not_null())
                    .col(ColumnDef::new(Model::Slug).string().not_null())
                    .col(ColumnDef::new(Model::Context).integer().not_null())
                    .col(ColumnDef::new(Model::InputPricing).json().not_null())
                    .col(ColumnDef::new(Model::OutputPricing).json().not_null())
                    .col(ColumnDef::new(Model::TrainingAt).timestamp().not_null())
                    .col(ColumnDef::new(Model::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Model::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-model-slug")
                    .table(Model::Table)
                    .col(Model::Slug)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-model-provider-id")
                    .from(Model::Table, Model::ProviderId)
                    .to(Provider::Table, Provider::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().if_exists().name("idx-model-slug").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Model::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Model {
    Table,
    Id,
    ProviderId,
    Name,
    Description,
    Slug,
    Context,
    InputPricing,
    OutputPricing,
    TrainingAt,
    CreatedAt,
    UpdatedAt,
}
