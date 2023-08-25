use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::super::domain::TankDomain;
use super::super::infrastructure::{TanksRepository, Tank};


pub struct TankService;
impl TankService {
    #[logfn(Trace)]
    pub fn get_tank_by_id(tank_id: String) -> Tank{
        TanksRepository::select_tank(tank_id)
    }

    #[logfn(Trace)]
    pub fn get_tanks(vessel_id: String) -> Vec<TankDomain>{
        let tanks = TanksRepository::select_tanks(vessel_id);
        let mut tank_domains = Vec::new();
        for tank in tanks{
            tank_domains.push(
                TankDomain::from_tank_model(tank)
            );
        }
        tank_domains
    }

    #[logfn(Trace)]
    pub fn add_tank(data: HashMap<String, Value>) -> Tank{
        TanksRepository::insert_tank(Tank::from_map(data))
    }

    #[logfn(Trace)]
    pub fn update_tank(tank: Tank) -> Tank {
        TanksRepository::update_tank(tank)
    }
}