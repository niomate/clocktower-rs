use clocktower_rs::establish_connection;
use clocktower_rs::models::*;
use diesel::prelude::*;

fn main() {
    use clocktower_rs::schema::worktime_entries::dsl::*;

    let connection = &mut establish_connection();

    let _ = diesel::delete(worktime_entries).execute(connection);
}
