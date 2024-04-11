// @generated automatically by Diesel CLI.

diesel::table! {
    worktime_entries (id) {
        id -> Int4,
        day -> Date,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        hadbreak -> Bool,
        finished -> Bool,
    }
}
