use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Provider::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Provider::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Provider::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Provider::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Provider::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Provider::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-provider-slug")
                    .table(Provider::Table)
                    .col(Provider::Slug)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-provider-created-at")
                    .table(Provider::Table)
                    .col(Provider::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-provider-created-at")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-provider-slug")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Provider::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Provider {
    Table,
    Id,
    Name,
    Slug,
    CreatedAt,
    UpdatedAt,
}
