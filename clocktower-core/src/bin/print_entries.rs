use clocktower_core::establish_connection;
use clocktower_core::models::*;
use diesel::prelude::*;
use tabled::Table;


fn main() {
    use clocktower_core::schema::worktime_entries::dsl::*;

    let connection = &mut establish_connection();

    let results = worktime_entries
        .limit(5)
        .select(WorktimeEntry::as_select())
        .load(connection)
        .expect("Error loading worktime entries");

    println!("{}", Table::new(results).to_string())
}
