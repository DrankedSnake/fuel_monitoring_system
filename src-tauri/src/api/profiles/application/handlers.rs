use std::collections::HashMap;

use serde_json::Value;


use super::super::infrastructure::{
    TankProfile,
    insert_tank_profile,
    select_tank_profiles,
    select_tank_profile,
};
use super::tank_profiles_service::create_tank_profiles_from_csv_file;



#[tauri::command]
pub fn add_tank_profile(tank_profile: HashMap<String, Value>) -> TankProfile{
    let tank_profile: TankProfile = TankProfile::from_map(tank_profile);
    insert_tank_profile(tank_profile)
}


#[tauri::command]
pub fn add_tank_profiles(file_path: String, tank_id: &str){
    create_tank_profiles_from_csv_file(file_path, tank_id);
}


#[tauri::command]
pub fn get_tank_profiles(tank_id: String) -> Vec<TankProfile>{
    select_tank_profiles(tank_id)
}

#[tauri::command]
pub fn get_tank_profile(tank_id: String, height: f64, trim: f64) -> TankProfile{
    println!("{}", tank_id);
    select_tank_profile(tank_id, height, trim)
}
