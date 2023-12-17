//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub task_version_id: Uuid,
    pub raw: Option<String>,
    pub content: String,
    pub role: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::task_version::Entity",
        from = "Column::TaskVersionId",
        to = "super::task_version::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TaskVersion,
}

impl Related<super::task_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
