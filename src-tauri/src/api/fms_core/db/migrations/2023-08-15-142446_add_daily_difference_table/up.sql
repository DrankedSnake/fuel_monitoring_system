-- Your SQL goes here
-- Your SQL goes here
create table daily_difference (
    id VARCHAR(64) PRIMARY KEY,
    vessel_id VARCHAR(64),
    volume FLOAT,
    mass FLOAT,
    date DATE,
    CONSTRAINT dd_vessel_id
    FOREIGN KEY(vessel_id) 
    REFERENCES vessel(id)
)