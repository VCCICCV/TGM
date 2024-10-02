use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: Option<i32>,
    pub user_id: i32,
    pub total_price: Decimal,
}
