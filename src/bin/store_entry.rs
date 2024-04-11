use chrono::prelude::*;
use clocktower_rs::{create_worktime_entry, establish_connection};
use diesel::dsl::now;

fn main() {
    let connection = &mut establish_connection();

    let entry = create_worktime_entry(
        connection,
        None,
        Some(Local::now().naive_local()),
        None,
        false,
    );

    println!("Saved draft entry {:?} with id {}", entry, entry.id)
}
