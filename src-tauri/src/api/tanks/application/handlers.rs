use std::collections::HashMap;
use serde_json::Value;

use crate::api::tanks::Tank;
use super::super::{select_tanks, insert_tank };


#[tauri::command]
pub fn get_tanks(vessel_id: String) -> Vec<Tank>{
    println!("{:?}", vessel_id);
    let tanks = select_tanks(vessel_id);
    return tanks;
}


#[tauri::command]
pub fn add_tank(tank: HashMap<String, Value>) -> Tank{
    println!("{:#?}", tank);
    let tank = Tank::from_map(tank);
    insert_tank(tank)
}