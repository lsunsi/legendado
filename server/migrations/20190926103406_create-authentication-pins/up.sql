create table authentication_pins (
	id serial primary key,
	user_id int not null references users(id),
	pin text not null,
	created_at timestamptz not null
);

create table authentication_pin_attempts (
	id serial primary key,
	user_id int not null references users(id),
	pin text not null,
	created_at timestamptz not null
);
