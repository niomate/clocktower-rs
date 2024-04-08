use std::env;

use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewEntry, WorktimeEntry};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_worktime_entry(
    conn: &mut PgConnection,
    date: Option<NaiveDate>,
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
) -> WorktimeEntry {
    use crate::schema::worktime_entries;

    let new_entry = NewEntry {
        date: date.unwrap_or_else(|| Utc::now().date_naive()),
        start_time: start_time.unwrap_or_else(|| Utc::now().naive_local()),
        end_time,
        finished: end_time.is_some(),
    };

    diesel::insert_into(worktime_entries::table)
        .values(&new_entry)
        .returning(WorktimeEntry::as_returning())
        .get_result(conn)
        .expect("Error storing new entry")
}
