use crate::models::UpdateEntry;
use chrono::prelude::*;
use clocktower_rs::*;
use diesel::prelude::*;

fn main() {
    use self::schema::worktime_entries::dsl::{day, finished, worktime_entries};

    let connection = &mut establish_connection();
    let now = Local::now();
    let today = now.date_naive();
    let end_time = now.naive_local();

    let _ = diesel::update(worktime_entries)
        .filter(day.eq(today))
        .filter(finished.eq(false))
        .set(UpdateEntry::new().end_time(end_time).finished(true).done())
        .execute(connection);
}
