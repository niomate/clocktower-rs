use self::models::WorktimeEntry;
use diesel::prelude::*;
use clocktower_rs::*;
use std::env::args;
use diesel::dsl::now;

fn main() {
    use self::schema::worktime_entries::dsl::{worktime_entries, finished};

    let connection = &mut establish_connection();
}
