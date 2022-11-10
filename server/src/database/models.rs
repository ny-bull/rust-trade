use sqlx::types::chrono;

#[derive(sqlx::Type, Debug)]
#[sqlx(rename = "bs")]
#[repr(i8)]
pub enum BS {
    Buy,
    Sell,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(rename = "status")]
#[repr(i8)]
pub enum Status {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TradeJob {
    pub id: i32,
    pub bs: BS,
    pub exchange: String,
    pub currency: String,
    pub order_id: Option<i32>,
    pub status: Status,
    pub amount: i32,
    pub create_at: chrono::DateTime<chrono::Utc>,
    pub update_at: chrono::DateTime<chrono::Utc>,
    pub trade_at: chrono::DateTime<chrono::Utc>,
}
