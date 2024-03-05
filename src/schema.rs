// @generated automatically by Diesel CLI.

diesel::table! {
    worktime_entries (id) {
        id -> Int4,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        finished -> Bool,
    }
}
