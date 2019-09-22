create table feedbacks (
	id serial primary key,
	key text not null,
	user_id int not null references users(id),
	subtitle_id int not null references subtitles(id)
);
