use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "key_feature")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::storage_type_key_feature::Entity")]
    StorageTypeKeyFeature,
}

impl Related<super::storage_type_key_feature::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StorageTypeKeyFeature.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
