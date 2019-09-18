create table downloads (
	id serial primary key,
	user_id int not null references users(id),
	subtitle_id int not null references subtitles(id)
);
