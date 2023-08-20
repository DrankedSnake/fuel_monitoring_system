use std::collections::HashMap;

use serde_json::Value;

use super::super::domain::TankProfile;
use super::service::TankProfileService;



#[tauri::command]
pub fn add_tank_profile(tank_profile: HashMap<String, Value>) -> TankProfile{
    TankProfileService::add_tank_profile(tank_profile)
}


#[tauri::command]
pub fn add_tank_profiles(file_path: String, tank_id: &str){
    println!("{}", file_path);
    TankProfileService::create_tank_profiles_from_csv_file(file_path, tank_id);
}


#[tauri::command]
pub fn get_tank_profiles(tank_id: String) -> Vec<TankProfile>{
    TankProfileService::get_tank_profiles(tank_id)
}


#[tauri::command]
pub fn get_tank_profile(tank_id: String, height: f64, trim: f64) -> TankProfile{
    TankProfileService::get_tank_profile(tank_id, height, trim)
}
