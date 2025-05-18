-- Your SQL goes here
alter table vessel add column dead_weight INT;
alter table vessel add column imo VARCHAR(16);
alter table vessel add column company VARCHAR(16);