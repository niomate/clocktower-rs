use clocktower_core::establish_connection;
use clocktower_core::models::*;
use diesel::prelude::*;

fn main() {
    use clocktower_core::schema::worktime_entries::dsl::*;

    let connection = &mut establish_connection();

    let _ = diesel::delete(worktime_entries).execute(connection);
}
