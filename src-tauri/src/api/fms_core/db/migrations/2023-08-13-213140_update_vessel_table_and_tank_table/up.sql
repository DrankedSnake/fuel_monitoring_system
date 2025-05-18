-- Your SQL goes here
alter table vessel add column year INT;
alter table tank rename column gas_type to fuel_type;