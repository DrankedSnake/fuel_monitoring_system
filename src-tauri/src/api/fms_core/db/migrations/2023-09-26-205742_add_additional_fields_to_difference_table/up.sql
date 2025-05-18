-- Your SQL goes here
alter table difference drop column density_coefficient_id;
alter table difference add column height FLOAT;
alter table difference add column trim FLOAT;
alter table difference add column temperature FLOAT;
alter table difference add column density FLOAT;