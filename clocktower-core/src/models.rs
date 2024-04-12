use chrono::prelude::*;
use diesel::prelude::*;
use tabled::Tabled;
use crate::format_duration;

fn display_datetime_option(o: &Option<NaiveDateTime>) -> String {
    match o {
        Some(s) => display_datetime(s),
        None => format!("--:--"),
    }
}

fn display_duration_option(o: &Option<chrono::Duration>) -> String {
    match o {
        Some(s) => format_duration(s),
        None => format!("--:--"),
    }
}

fn display_datetime(s: &NaiveDateTime) -> String {
    format!("{}", s.format("%H:%Mh"))
}

#[derive(Tabled)]
pub struct EntryTable {
    pub day: NaiveDate,
    #[tabled(display_with = "display_datetime")]
    pub start_time: NaiveDateTime,
    #[tabled(display_with = "display_datetime_option")]
    pub end_time: Option<NaiveDateTime>,
    #[tabled(display_with = "display_duration_option")]
    pub duration: Option<chrono::Duration>,
    pub hadbreak: bool,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::worktime_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorktimeEntry {
    pub id: i32,
    pub day: NaiveDate,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub finished: bool,
    pub hadbreak: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::worktime_entries)]
pub struct NewEntry {
    pub day: NaiveDate,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub finished: bool,
    pub hadbreak: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::worktime_entries)]
pub struct UpdateEntry {
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub finished: Option<bool>,
    pub hadbreak: Option<bool>,
}

impl UpdateEntry {
    pub fn new() -> EntryBuilder {
        EntryBuilder::default()
    }
}

pub struct EntryBuilder {
    _start_time: Option<NaiveDateTime>,
    _end_time: Option<NaiveDateTime>,
    _finished: Option<bool>,
    _hadbreak: Option<bool>,
}

impl Default for EntryBuilder {
    fn default() -> Self {
        EntryBuilder {
            _start_time: None,
            _end_time: None,
            _finished: None,
            _hadbreak: None,
        }
    }
}

impl EntryBuilder {
    pub fn start_time(&mut self, date: NaiveDateTime) -> &mut Self {
        self._start_time = Some(date);
        self
    }
    pub fn end_time(&mut self, date: NaiveDateTime) -> &mut Self {
        self._end_time = Some(date);
        self
    }
    pub fn finished(&mut self, val: bool) -> &mut Self {
        self._finished = Some(val);
        self
    }
    pub fn hadbreak(&mut self, val: bool) -> &mut Self {
        self._hadbreak = Some(val);
        self
    }
    pub fn finished_at(&mut self, date: NaiveDateTime) -> &mut Self {
        self._end_time = Some(date);
        self._finished = Some(true);
        self
    }
    pub fn done(&self) -> UpdateEntry {
        UpdateEntry {
            start_time: self._start_time,
            end_time: self._end_time,
            finished: self._finished,
            hadbreak: self._hadbreak,
        }
    }
}
