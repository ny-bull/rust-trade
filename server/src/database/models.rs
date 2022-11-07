#[derive(sqlx::Type)]
#[repr(i16)]
pub enum BS {
    Buy,
    Sell,
}

#[derive(sqlx::Type)]
#[repr(i16)]
pub enum Status {
    Pending,
    Running,
    Completed,
    Failed,
}

pub struct TradeJob {
    pub id: i32,
    pub bs: BS,
    pub exchange: String,
    pub currency: String,
    pub order_id: Option<i32>,
    pub amount: i32,
    pub status: Status,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
    pub trade_at: chrono::NaiveDateTime,
}
