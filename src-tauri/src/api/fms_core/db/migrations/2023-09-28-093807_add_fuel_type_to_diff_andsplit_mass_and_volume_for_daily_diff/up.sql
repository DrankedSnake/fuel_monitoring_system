-- Your SQL goes here
alter table difference add column fuel_type Varchar(16);
alter table daily_difference drop column volume;
alter table daily_difference drop column mass;
alter table daily_difference add column hfo_volume FLOAT;
alter table daily_difference add column hfo_mass FLOAT;
alter table daily_difference add column mgo_volume FLOAT;
alter table daily_difference add column mgo_mass FLOAT;