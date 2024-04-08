create table worktime_entries (
	id serial primary key,
	day date not null,
	start_time timestamp not null,
	end_time timestamp,
	break boolean not null default true,
	finished boolean not null default false
)
