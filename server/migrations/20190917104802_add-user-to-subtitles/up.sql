alter table subtitles
add user_id int not null;

alter table subtitles
add constraint fk_subtitles_user
foreign key (user_id)
references users(id);
