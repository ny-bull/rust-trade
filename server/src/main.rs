pub mod database;
use database::DbExt;

#[tokio::main]
async fn main() {
    let pool = database::get_connection().await;
}

#[cfg(test)]
mod test;
