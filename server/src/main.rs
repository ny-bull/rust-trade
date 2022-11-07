pub mod database;
use std::process;
#[tokio::main]
async fn main() {
    let pool = database::get_connection().await;
    if let Err(err) = sqlx::query!(
        r#"
        INSERT INTO hoge(
            id,
            name
        )
        VALUES(
            1,
            "yukarisann kawaii yatta-"
        );
    "#
    )
    .execute(&pool)
    .await
    {
        eprint!("Error: sqlx said {}\n", err);
        process::exit(1);
    }
}
