use std::collections::HashMap;

use serde_json::Value;

use super::super::infrastructure::{
    select_density_coefficients,
    insert_density_coefficient,
    insert_density_coefficients,
    DensityCoefficient,
};


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