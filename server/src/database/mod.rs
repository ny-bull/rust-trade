use dotenvy::dotenv;
use sqlx::mysql::{self};
use sqlx::{mysql::MySqlPool, Pool};
use std::{env, process};
pub mod models;
use async_trait::async_trait;

use self::models::TradeJob;
pub async fn get_connection() -> Pool<mysql::MySql> {
    dotenv().ok();
    let database_url = match env::var("DATABASE_URL") {
        Ok(ok) => ok,
        Err(err) => {
            eprint!("Error: std::env said, {}\n", err);
            process::exit(1);
        }
    };

    let pool = match MySqlPool::connect(&database_url).await {
        Ok(ok) => ok,
        Err(err) => {
            eprint!("Error: sqlx said, {}\n", err);
            process::exit(1);
        }
    };
    pool
}

#[async_trait]
pub trait DbExt {
    async fn select(self, sql: String) -> Vec<models::TradeJob>;
}

