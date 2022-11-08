use sqlx::types::chrono;


#[derive(sqlx::Type,Debug)]
pub enum BS {
    Buy,
    Sell,
}

#[derive(sqlx::Type,Debug)]
pub enum Status {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct TradeJob {
    pub id: i32,
    pub bs: BS,
    pub exchange: String,
    pub currency: String,
    pub order_id: Option<i32>,
    pub amount: i32,
    pub create_at: chrono::DateTime<chrono::Utc>,
    pub update_at: chrono::DateTime<chrono::Utc>,
    pub trade_at: chrono::DateTime<chrono::Utc>,
}
