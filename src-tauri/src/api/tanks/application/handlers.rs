use std::collections::HashMap;

use serde_json::Value;

use super::super::infrastructure::{
    Tank,
    select_tanks,
    insert_tank,
};



#[tauri::command]
pub fn get_tanks(vessel_id: String) -> Vec<Tank>{
    select_tanks(vessel_id)
}


#[tauri::command]
pub fn add_tank(tank: HashMap<String, Value>) -> Tank{
    let tank = Tank::from_map(tank);
    insert_tank(tank)
}
