use std::env;

use anyhow::Result;
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{EntryTable, NewEntry, WorktimeEntry};
use tabled::Table;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn insert_worktime_entry(
    conn: &mut PgConnection,
    date: Option<NaiveDate>,
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
    hadbreak: bool,
) -> Result<bool> {
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
        .execute(conn)
        .map(|nrows| nrows == 1 as usize)
        .map_err(|err| err.into())
}

fn generic_update(
    conn: &mut PgConnection,
    date: NaiveDate,
    update: &models::UpdateEntry,
) -> Result<bool> {
    use self::schema::worktime_entries::dsl::{day, worktime_entries};

    diesel::update(worktime_entries)
        .filter(day.eq(date))
        .set(update)
        .execute(conn)
        .map(|nrows| nrows == 1 as usize)
        .map_err(|err| err.into())
}

pub fn set_workday_finished(
    conn: &mut PgConnection,
    date: NaiveDate,
    end_time: NaiveDateTime,
    hadbreak: bool,
) -> Result<bool> {
    generic_update(
        conn,
        date,
        &models::UpdateEntry::new()
            .finished_at(end_time)
            .hadbreak(hadbreak)
            .done()
            .into(),
    )
}

pub fn set_workday_finished_now(
    conn: &mut PgConnection,
    date: NaiveDate,
    hadbreak: bool,
) -> Result<bool> {
    set_workday_finished(conn, date, Local::now().naive_local(), hadbreak)
}

pub fn update_start_time(
    conn: &mut PgConnection,
    date: NaiveDate,
    start_time: NaiveDateTime,
) -> Result<bool> {
    generic_update(
        conn,
        date,
        &models::UpdateEntry::new()
            .start_time(start_time)
            .done()
            .into(),
    )
}

pub fn set_break(conn: &mut PgConnection, date: NaiveDate) -> Result<bool> {
    generic_update(
        conn,
        date,
        &models::UpdateEntry::new().hadbreak(true).done().into(),
    )
}

pub struct WorktimeSummary {
    num_workdays: u16,
    pub total_duration: chrono::Duration,
}

pub fn format_duration(duration: &chrono::Duration) -> String {
    let seconds = duration.num_seconds();
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if seconds < 0 {
        format!("-{}h {:02}m", -hours % 60, -minutes % 60)
    } else {
        format!("{}h {:02}m", hours % 60, minutes % 60)
    }
}

impl WorktimeSummary {
    pub fn overtime(&self) -> chrono::Duration {
        self.total_duration - chrono::Duration::hours(8 * self.num_workdays as i64)
    }
}

pub fn sum_worktimes(conn: &mut PgConnection) -> Result<WorktimeSummary> {
    use self::schema::worktime_entries::dsl::{finished, worktime_entries};
    let entries = worktime_entries
        .filter(finished.eq(true))
        .select(WorktimeEntry::as_select())
        .load(conn)?;

    let total_duration = entries
        .iter()
        .filter_map(|entry| {
            entry.end_time.map(|et| {
                let duration = et - entry.start_time;
                if entry.hadbreak {
                    duration - chrono::Duration::minutes(30)
                } else {
                    duration
                }
            })
        })
        .sum::<chrono::Duration>();

    let num_workdays = entries.iter().count() as u16;

    Ok(WorktimeSummary {
        num_workdays,
        total_duration,
    })
}

pub fn delete_all_entries(conn: &mut PgConnection) -> Result<bool> {
    use self::schema::worktime_entries::dsl::worktime_entries;
    diesel::delete(worktime_entries)
        .execute(conn)
        .map(|nrows| nrows > 0)
        .map_err(|err| err.into())
}

pub fn delete_entry(conn: &mut PgConnection, date: NaiveDate) -> Result<bool> {
    use self::schema::worktime_entries::dsl::{day, worktime_entries};
    diesel::delete(worktime_entries.filter(day.eq(date)))
        .execute(conn)
        .map(|nrows| nrows > 0)
        .map_err(|err| err.into())
}

pub fn print_entries(conn: &mut PgConnection) -> Result<()> {
    use self::schema::worktime_entries::dsl::worktime_entries;
    let results: Vec<EntryTable> = worktime_entries
        .select(WorktimeEntry::as_select())
        .load(conn)
        .expect("Error loading worktime entries")
        .iter()
        .map(|entry| EntryTable {
            day: entry.day,
            start_time: entry.start_time,
            end_time: entry.end_time,
            duration: entry.end_time.map(|e| e - entry.start_time),
            hadbreak: entry.hadbreak,
        })
        .collect();

    Ok(println!("{}", Table::new(results).to_string()))
}
