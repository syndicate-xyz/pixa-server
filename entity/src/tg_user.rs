use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tg_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    // #[serde(skip_deserializing)]
    pub id: i32,
    pub telegram_id: i64,
    pub username: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
