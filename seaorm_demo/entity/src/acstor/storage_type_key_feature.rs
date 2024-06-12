use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    async_graphql :: SimpleObject,
)]
#[sea_orm(table_name = "storage_type_key_feature")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub storage_type_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub key_feature_id: i32,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::storage_type::Entity",
        from = "Column::StorageTypeId",
        to = "super::storage_type::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    StorageType,
    #[sea_orm(
        belongs_to = "super::key_feature::Entity",
        from = "Column::KeyFeatureId",
        to = "super::key_feature::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    KeyFeature,
}

impl Related<super::storage_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StorageType.def()
    }
}

impl Related<super::key_feature::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KeyFeature.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
