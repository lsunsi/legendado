create table feedbacks (
	id serial primary key,
	key text not null,
	user_id int not null references users(id),
	subtitle_id int not null references subtitles(id)
);

create unique index feedbacks_user_subtitle
on feedbacks (user_id, subtitle_id);
