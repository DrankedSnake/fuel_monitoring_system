use std::collections::HashMap;

use serde_json::Value;

use super::super::infrastructure::TanksRepository;
use super::super::domain::Tank;


pub struct TankService;
impl TankService {
    pub fn get_tank_by_id(tank_id: String) -> Tank{
        println!("{}", tank_id);
        TanksRepository::select_tank(tank_id)
    }

    pub fn get_tanks(vessel_id: String) -> Vec<Tank>{
        TanksRepository::select_tanks(vessel_id)
    }

    pub fn add_tank(data: HashMap<String, Value>) -> Tank{
        TanksRepository::insert_tank(Tank::from_map(data))
    }

    pub fn update_tank(tank: Tank) -> Tank {
        TanksRepository::update_tank(tank)
    }
}