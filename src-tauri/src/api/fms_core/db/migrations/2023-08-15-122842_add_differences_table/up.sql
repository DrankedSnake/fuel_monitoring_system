-- Your SQL goes here
create table difference (
    id VARCHAR(64) PRIMARY KEY,
    tank_id VARCHAR(64),
    volume FLOAT,
    mass FLOAT,
    difference_type VARCHAR(32)
)