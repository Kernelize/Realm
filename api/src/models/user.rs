use chrono::NaiveDateTime;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use sonyflake::Sonyflake;

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>, // 可选字段
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub fn generate_snowflake_id() -> u64 {
    let mut generator = Sonyflake::new().unwrap();
    generator.next_id().unwrap()
}

fn mff() {
    let sf = Sonyflake::new().unwrap();
    let id = sf.next_id().unwrap();
}
