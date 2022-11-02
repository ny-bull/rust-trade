// @generated automatically by Diesel CLI.

diesel::table! {
    trade_jobs (id) {
        id -> Integer,
        bs -> Tinyint,
        exchange -> Varchar,
        currency -> Varchar,
        order_id -> Nullable<Integer>,
        amount -> Integer,
        status -> Bool,
        create_at -> Datetime,
        update_at -> Datetime,
        trade_at -> Datetime,
    }
}
