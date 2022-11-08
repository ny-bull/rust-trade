use std::process;

use super::*;

#[tokio::test]
async fn insert_job(){
    let pool = database::get_connection().await;
    if let Err(err) = sqlx::query!(
        r#"
        INSERT INTO trade_jobs(
            bs,
            exchange,
            currency,
            order_id,
            amount,
            status,
            create_at,
            update_at,
            trade_at
        )
        VALUES(
            'Buy',
            'Binance',
            'BTCUSD',
            NULL,
            10,
            'Pending',
            now(),
            now(),
            now()
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
