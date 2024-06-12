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
#[sea_orm(table_name = "milestone_key_feature")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub milestone_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub key_feature_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::milestone::Entity",
        from = "Column::MilestoneId",
        to = "super::milestone::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Milestone,
}

impl Related<super::milestone::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Milestone.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
