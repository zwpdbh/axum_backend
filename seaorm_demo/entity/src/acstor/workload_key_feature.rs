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
#[sea_orm(table_name = "workload_key_feature")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub workload_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub key_feature_id: i32,
}

/// `from` means one end of the relation in current Model.
/// `to` means another end of the relation in the other Model
/// `belongs_to` express the connection mapping between these two.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        from = "Column::WorkloadId",
        to = "super::workload::Column::Id",
        belongs_to = "super::workload::Entity"
    )]
    Workload,

    #[sea_orm(
        from = "Column::KeyFeatureId",
        to = "super::key_feature::Column::Id",
        belongs_to = "super::key_feature::Entity"
    )]
    KeyFeature,
}

impl Related<super::workload::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workload.def()
    }
}

impl Related<super::key_feature::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KeyFeature.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
