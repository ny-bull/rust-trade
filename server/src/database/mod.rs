use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_jobs(con: &mut MysqlConnection) -> Vec<models::TradeJob> {
    use self::schema::trade_jobs::dsl::*;

    let results = trade_jobs
        .filter(status.eq(true))
        .load::<models::TradeJob>(con)
        .expect("err");
    results
}
