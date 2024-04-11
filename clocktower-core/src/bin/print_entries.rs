use clocktower_core::establish_connection;
use clocktower_core::models::*;
use clocktower_core::sum_worktimes;
use diesel::prelude::*;
use tabled::Table;

fn main() -> anyhow::Result<()> {
    use clocktower_core::schema::worktime_entries::dsl::*;

    let connection = &mut establish_connection();

    let results = worktime_entries
        .limit(5)
        .select(WorktimeEntry::as_select())
        .load(connection)
        .expect("Error loading worktime entries");

    let total_worktime = sum_worktimes(connection)?;

    let seconds = total_worktime.num_seconds() % 60;
    let minutes = (total_worktime.num_seconds() / 60) % 60;
    let hours = (total_worktime.num_seconds() / 60) / 60;

    println!("Total worktime: {}h {}m {}s", hours, minutes, seconds);
    println!("{}", Table::new(results).to_string());

    Ok(())
}
