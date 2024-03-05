use clocktower_rs::{establish_connection, create_worktime_entry};
use clocktower_rs::models::*;
use std::io::{stdin, Read};
use chrono::prelude::*;

fn main() {
    let connection = &mut establish_connection();

    let entry = create_worktime_entry(connection, None, Some(Utc::now().naive_local()));

    println!("Saved draft entry {:?} with id {}", entry, entry.id)
}
