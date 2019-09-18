alter table subtitles
drop constraint fk_subtitles_user;

alter table subtitles
drop column user_id;
