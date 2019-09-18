alter table users
alter column email drop not null;

alter table subtitles
alter column raw_name drop not null;

alter table subtitles
alter column mime drop not null;

alter table subtitles
alter column content drop not null;
