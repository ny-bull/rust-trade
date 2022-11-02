// @generated automatically by Diesel CLI.

diesel::table! {
    trade_jobs (id) {
        id -> Integer,
        bs -> Tinyint,
        trade_type -> Varchar,
        currency -> Varchar,
        origin_id -> Nullable<Integer>,
        amount -> Integer,
        status -> Bool,
        create_at -> Datetime,
        update_at -> Datetime,
        trade_at -> Datetime,
    }
}
