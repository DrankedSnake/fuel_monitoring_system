-- Your SQL goes here
alter table difference add column date_created TIMESTAMP;
alter table difference add density_coefficient_id VARCHAR(64);
alter table difference add constraint f_tank_id FOREIGN KEY(tank_id) REFERENCES tank(id);
alter table difference add constraint f_density_coefficient_id FOREIGN KEY(density_coefficient_id) REFERENCES density_coefficient(id);