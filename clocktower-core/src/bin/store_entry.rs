use chrono::prelude::*;
use clocktower_core::{create_worktime_entry, establish_connection};

fn main() -> anyhow::Result<()> {
    let connection = &mut establish_connection();

    let entry = create_worktime_entry(
        connection,
        None,
        Some(Local::now().naive_local()),
        None,
        false,
    )?;

    println!("Saved draft entry {:?} with id {}", entry, entry.id);

    Ok(())
}
