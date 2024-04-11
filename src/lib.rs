use std::env;

use anyhow::Result;
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
    hadbreak: bool,
) -> Result<WorktimeEntry> {
    use crate::schema::worktime_entries;

    let new_entry = NewEntry {
        day: date.unwrap_or_else(|| Local::now().date_naive()),
        start_time: start_time.unwrap_or_else(|| Local::now().naive_local()),
        end_time,
        finished: end_time.is_some(),
        hadbreak,
    };

    diesel::insert_into(worktime_entries::table)
        .values(&new_entry)
        .returning(WorktimeEntry::as_returning())
        .get_result(conn)
        .map_err(|err| err.into())
}

pub fn set_workday_finished(
    conn: &mut PgConnection,
    date: NaiveDate,
    _hadbreak: bool,
) -> Result<usize> {
    use self::schema::worktime_entries::dsl::{day, finished, worktime_entries};

    let now = Local::now();
    let end_time = now.naive_local();

    diesel::update(worktime_entries)
        .filter(day.eq(date))
        .filter(finished.eq(false))
        .set(
            &models::UpdateEntry::new()
                .finished_at(end_time)
                .done(),
        )
        .execute(conn)
        .map_err(|err| err.into())
}
