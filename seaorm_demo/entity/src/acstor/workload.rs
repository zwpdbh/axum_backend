use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "workload")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub storage_type_id: i32,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        from = "Column::StorageTypeId",
        to = "super::storage_type::Column::Id",
        belongs_to = "super::storage_type::Entity"
    )]
    StorageType,
}

impl ActiveModelBehavior for ActiveModel {}
