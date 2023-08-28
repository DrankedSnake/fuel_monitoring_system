-- Your SQL goes here
create table height_correction (
    id VARCHAR(64) PRIMARY KEY,
    tank_id VARCHAR(64),
    trim FLOAT,
    correction FLOAT,
    CONSTRAINT t_tank_id
    FOREIGN KEY(tank_id) 
    REFERENCES tank(id)
);

alter table tank add column correction_type VARCHAR(16);

alter table difference drop column difference_type;