use std::collections::HashMap;

use log_derive::{logfn, logfn_inputs};
use serde_json::Value;

use super::super::domain::TankProfile;
use super::service::TankProfileService;


#[logfn(Trace)]
#[tauri::command]
pub fn add_tank_profile(tank_profile: HashMap<String, Value>) -> TankProfile{
    TankProfileService::add_tank_profile(tank_profile)
}

#[logfn_inputs(INFO, fmt = "Read csv files {} for tank {}")]
#[tauri::command]
pub fn add_tank_profiles(file_path: String, tank_id: &str){
    TankProfileService::create_tank_profiles_from_csv_file(file_path, tank_id).expect("No such file found");
}

#[tauri::command]
pub fn get_tank_profiles(search_form: HashMap<String, Value>) -> Vec<TankProfile>{
    TankProfileService::get_tank_profiles(search_form)
}

#[logfn(Trace)]
#[tauri::command]
pub fn get_tank_profile(tank_id: String, height: String, trim: String) -> Option<TankProfile>{
    let height = height.replace("\"", "").parse::<f64>().unwrap();
    let trim = trim.replace("\"", "").parse::<f64>().unwrap();

    TankProfileService::get_tank_profile(tank_id, height, trim)
}

#[logfn(Trace)]
#[tauri::command]
pub fn get_tank_profiles_amount(search_form: HashMap<String, Value>) -> i64{
    TankProfileService::get_tank_profiles_amount(search_form)
}
