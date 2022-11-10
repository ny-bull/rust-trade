use dotenvy::{dotenv, Result};
use sqlx::mysql::{self};
use sqlx::types::chrono;
use sqlx::{mysql::MySqlPool, Pool};
use std::{env, process};
pub mod models;
use async_trait::async_trait;

use self::models::{TradeJob, BS};
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
    async fn select(self, sql: String) -> anyhow::Result<()>;
}

#[async_trait]
impl DbExt for Pool<mysql::MySql> {
    async fn select(self, sql: String) -> anyhow::Result<()> {
        //let recs = sqlx::query!(
            //r#"
            //SELECT id,bs 
            //FROM trade_jobs
            //ORDER BY id
        //"#
        //)
        //.fetch_all(&self)
        //.await?;
        //for rec in recs {
            //println!("[{}],{}", rec.id, rec.bs);
        //}

        let stream = sqlx::query_as::<_, TradeJob>("SELECT * FROM trade_jobs")
            .fetch_all(&self)
            .await?;
        for job in stream {
            println!("{:?}:{}", job.bs,job.id);
        }

        Ok(())
    }
}
