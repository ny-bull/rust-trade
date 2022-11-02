use diesel::prelude::*;

#[derive(Queryable)]
pub struct TradeJob {
    pub id: i32,
    pub bs: i8,
    pub exchange: String,
    pub currency: String,
    pub order_id: Option<i32>,
    pub amount: i32,
    pub status: bool,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
    pub trade_at: chrono::NaiveDateTime,
}
