use diesel::prelude::*;

#[derive(Queryable)]
pub struct TradeJob {
    pub id: i32,
    pub bs: i8,
    pub trade_type: String,
    pub currency: String,
    pub origin_id: Option<i32>,
    pub amount: i32,
    pub status: bool,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
    pub trade_at: chrono::NaiveDateTime,
}
