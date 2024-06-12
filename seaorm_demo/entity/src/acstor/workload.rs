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
    pub release_version: String,
}

/// A workload belongs to one type of storage_type.
/// It could also cover one or more features
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        from = "Column::StorageTypeId",
        to = "super::storage_type::Column::Id",
        belongs_to = "super::storage_type::Entity"
    )]
    StorageType,

    #[sea_orm(has_many = "super::workload_key_feature::Entity")]
    WorkloadKeyFeature,
}

/// Enable `find_with_related` feature.
/// For instance, we want to show a workload and its related key_feature.
impl Related<super::workload_key_feature::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkloadKeyFeature.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
