create table worktime_entries (
	id serial primary key,
	day date unique not null,
	start_time timestamp not null,
	end_time timestamp,
	hadbreak boolean not null default true,
	finished boolean not null default false
)
