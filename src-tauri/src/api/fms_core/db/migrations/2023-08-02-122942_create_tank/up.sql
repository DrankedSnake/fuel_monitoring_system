-- Your SQL goes here
create table tank(
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(32) not NULL,
    available_volume FLOAT,
    current_volume FLOAT,
    previous_volume FLOAT,
    gas_type VARCHAR(32),
    vessel_id VARCHAR(64)
);