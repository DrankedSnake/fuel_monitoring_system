-- Your SQL goes here
alter table tank rename column available_volume to volume;
alter table tank add column safe_volume FLOAT;
alter table tank add column current_mass FLOAT;
alter table tank add column previous_mass FLOAT;
alter table tank add column tank_type VARCHAR(32);