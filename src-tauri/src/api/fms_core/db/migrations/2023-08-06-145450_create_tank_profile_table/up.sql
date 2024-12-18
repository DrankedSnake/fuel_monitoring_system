create table tank_profile(
    id VARCHAR(64) PRIMARY KEY,
    tank_id VARCHAR(64),
    height FLOAT,
    trim FLOAT,
    volume FLOAT,
    CONSTRAINT profile_tank_id
      FOREIGN KEY(tank_id) 
	  REFERENCES tank(id)
);