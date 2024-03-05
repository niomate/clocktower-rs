use clocktower_rs::establish_connection;
use clocktower_rs::models::*;
use diesel::prelude::*;

fn main() {
    use clocktower_rs::schema::worktime_entries::dsl::*;

    let connection = &mut establish_connection();

    let results = worktime_entries
        .filter(finished.eq(true))
        .limit(5)
        .select(WorktimeEntry::as_select())
        .load(connection)
        .expect("Error loading worktime entries");

    println!("Displaying {} worktime entries", results.len());

    for entry in results {
        println!("{}", entry.start_time);
        println!("-----------\n");
        println!(
            "{}",
            entry
                .end_time
                .map_or_else(|| "No end time.".to_owned(), |x| x.to_string())
        );
    }
}
