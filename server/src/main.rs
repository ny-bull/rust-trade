pub mod database;
use database::DbExt;

#[tokio::main]
async fn main() {
    let pool = database::get_connection().await;
    println!("select before");
    let result = pool.select(String::from("")).await;
    match result{
        Ok(s) => println!("success"),
        Err(s) => println!("error {}",s)
    }
    
    println!("select after");
}

#[cfg(test)]
mod test;
