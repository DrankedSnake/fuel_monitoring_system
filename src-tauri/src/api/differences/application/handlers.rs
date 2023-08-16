use std::collections::HashMap;

use serde_json::Value;

use super::super::infrastructure::{
    Difference,
    insert_difference,
    select_differences,
};


#[tauri::command]
pub fn add_difference(data: HashMap<String, Value>) -> Difference{
    let tank_profile = 
    insert_difference(Difference::from_map(data))
}

#[tauri::command]
pub fn get_differences(tank_id: String) -> Vec<Difference>{
    select_differences(tank_id)
}