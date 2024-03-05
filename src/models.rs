use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::worktime_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct WorktimeEntry {
    pub id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub finished: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::worktime_entries)]
pub struct NewEntry {
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub finished: bool
}
