use self::models::WorktimeEntry;
use diesel::prelude::*;
use clocktower_rs::*;
use std::env::args;

fn main() {
    use self::schema::worktime_entries::dsl::{worktime_entries, finished};

    let connection = &mut establish_connection();

    let entry = diesel::update(worktime_entries.find())
}
