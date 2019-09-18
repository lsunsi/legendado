create table users (
	id serial,
	email text
);

create unique index users_email
on users (email);
