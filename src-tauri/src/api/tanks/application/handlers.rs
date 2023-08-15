use std::collections::HashMap;
use serde_json::Value;
use super::create_tank_profiles_from_csv_file;

use crate::api::tanks::{
    Tank, 
    TankProfile,
    DensityCoefficient,
    select_tanks,
    select_density_coefficients_for_temperature,
    select_density_coefficients,
    select_tank_profiles,
    insert_tank,
    insert_tank_profile,
    insert_density_coefficient,
    insert_density_coefficients,
};


#[tauri::command]
pub fn get_tanks(vessel_id: String) -> Vec<Tank>{
    let tanks = select_tanks(vessel_id);
    return tanks;
}


#[tauri::command]
pub fn add_tank(tank: HashMap<String, Value>) -> Tank{
    let tank = Tank::from_map(tank);
    insert_tank(tank)
}



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
pub fn get_density_coefficients() -> Vec<DensityCoefficient>{
    select_density_coefficients()
}


#[tauri::command]
pub fn add_density_coefficient(density_coefficient: HashMap<String, Value>) -> DensityCoefficient{
    insert_density_coefficient(DensityCoefficient::from_map(density_coefficient))
}


#[tauri::command]
pub fn add_density_coefficients(file_path: &str) -> Vec<DensityCoefficient>{
    insert_density_coefficients(DensityCoefficient::from_csv(file_path))
}