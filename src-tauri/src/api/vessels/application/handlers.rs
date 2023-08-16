use std::collections::HashMap;

use serde_json::Value;

use super::super::infrastructure::{
    select_vessels,
    insert_vessel,
    Vessel,
};


#[tauri::command]
pub fn get_vessels() -> Vec<Vessel>{
    let vessels = select_vessels();
    vessels
}


#[tauri::command]
pub fn add_vessel(vessel: HashMap<String, Value>) -> Vessel{
    let vessel = Vessel::from_map(vessel);
    insert_vessel(vessel)
}