use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::worktime_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
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
    pub end_time: Option<NaiveDateTime>,
    pub finished: Option<bool>,
    pub hadbreak: Option<bool>,
}

pub struct UpdateEntryBuilder {
    _end_time: Option<NaiveDateTime>,
    _finished: Option<bool>,
    _hadbreak: Option<bool>,
}

impl UpdateEntry {
    pub fn new() -> UpdateEntryBuilder {
        UpdateEntryBuilder {
            _end_time: None,
            _finished: None,
            _hadbreak: None,
        }
    }
}

impl UpdateEntryBuilder {
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
            end_time: self._end_time,
            finished: self._finished,
            hadbreak: self._hadbreak,
        }
    }
}
