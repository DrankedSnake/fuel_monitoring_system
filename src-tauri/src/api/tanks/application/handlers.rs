use std::collections::HashMap;

use serde_json::Value;

use super::super::domain::Tank;
use super::service::TankService;


#[tauri::command]
pub fn get_tanks(vessel_id: String) -> Vec<Tank>{
    TankService::get_tanks(vessel_id)
}

#[tauri::command]
pub fn get_tank(tank_id: String) -> Tank{
    TankService::get_tank_by_id(tank_id)
}


#[tauri::command]
pub fn add_tank(tank: HashMap<String, Value>) -> Tank{
    TankService::add_tank(tank)
}
