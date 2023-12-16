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
                    .as_enum(UserRole::Table)
                    .values([UserRole::Admin, UserRole::User])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Salt).string().not_null())
                    .col(
                        ColumnDef::new(User::Role)
                            .enumeration(UserRole::Table, [UserRole::Admin, UserRole::User])
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user-email")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user-created-at")
                    .table(User::Table)
                    .col(User::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-user-created-at")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().if_exists().name("idx-user-email").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Username,
    Password,
    Salt,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
enum UserRole {
    Table,
    #[iden = "Admin"]
    Admin,
    #[iden = "User"]
    User,
}
