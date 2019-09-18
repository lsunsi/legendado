alter table users
alter column email set not null;

alter table subtitles
alter column raw_name set not null;

alter table subtitles
alter column mime set not null;

alter table subtitles
alter column content set not null;
