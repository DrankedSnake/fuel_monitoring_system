-- Your SQL goes here
alter table tank add constraint f_vessel_id FOREIGN KEY(vessel_id) REFERENCES vessel(id);