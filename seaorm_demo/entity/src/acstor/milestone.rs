use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "milestone")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::milestone_key_feature::Entity")]
    MilestoneKeyFeature,
}

/// Enable `find_with_related` feature.
/// For instance, we want to show a key_feature and its related milestone
impl Related<super::workload_key_feature::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MilestoneKeyFeature.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
