use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_user_table::User;
use crate::m20231216_063415_create_provider_table::Provider;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ApiKey::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ApiKey::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ApiKey::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(ApiKey::ProviderId).uuid().not_null())
                    .col(ColumnDef::new(ApiKey::Name).string().not_null())
                    .col(ColumnDef::new(ApiKey::Key).string().not_null())
                    .col(ColumnDef::new(ApiKey::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ApiKey::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-api-key-provider-id")
                    .from(ApiKey::Table, ApiKey::ProviderId)
                    .to(Provider::Table, Provider::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-api-key-owner-id")
                    .from(ApiKey::Table, ApiKey::OwnerId)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiKey::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ApiKey {
    Table,
    Id,
    ProviderId,
    OwnerId,
    Name,
    Key,
    CreatedAt,
    UpdatedAt,
}
