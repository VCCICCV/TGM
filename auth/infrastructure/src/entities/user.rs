//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub role: Option<i32>,
    pub salt: String,
    pub avatar: Option<String>,
    pub is_deleted: bool,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
