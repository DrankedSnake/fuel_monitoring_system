use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::super::domain::TankDomain;
use super::super::infrastructure::Tank;
use super::service::TankService;

#[logfn(Trace)]
#[tauri::command]
pub fn get_tanks(vessel_id: String) -> Vec<TankDomain> {
    TankService::get_tanks(vessel_id)
}
#[logfn(Trace)]
#[tauri::command]
pub fn get_tank(tank_id: String) -> Tank {
    TankService::get_tank_by_id(tank_id)
}

#[logfn(Trace)]
#[tauri::command]
pub fn add_tank(tank: HashMap<String, Value>) -> Tank {
    TankService::add_tank(tank)
}
