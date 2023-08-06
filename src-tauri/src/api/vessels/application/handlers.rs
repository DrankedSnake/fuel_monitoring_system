use std::collections::HashMap;

use serde_json::Value;

use crate::api::vessels::{infrastructure::Vessel, insert_vessel, select_vessels};


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