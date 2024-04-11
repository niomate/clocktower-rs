use chrono::prelude::*;
use clocktower_core::{create_worktime_entry, establish_connection};

fn main() -> anyhow::Result<()> {
    let connection = &mut establish_connection();

    let _ = create_worktime_entry(
        connection,
        None,
        Some(Local::now().naive_local()),
        None,
        false,
    )?;

    Ok(())
}
