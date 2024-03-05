create table worktime_entries (
	id serial primary key,
	start_time timestamp not null,
	end_time timestamp,
	finished boolean not null default false
)
