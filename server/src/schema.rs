// @generated automatically by Diesel CLI.

diesel::table! {
    trade_jobs (id) {
        id -> Integer,
        BS -> Nullable<Tinyint>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        currency -> Nullable<Varchar>,
        origin_id -> Nullable<Integer>,
        amount -> Nullable<Integer>,
        status -> Nullable<Tinyint>,
        create_at -> Nullable<Datetime>,
        update_at -> Nullable<Datetime>,
        trade_at -> Nullable<Datetime>,
    }
}
