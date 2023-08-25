use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::service::DensityCoefficientService;
use super::super::domain::DensityCoefficient;


#[tauri::command]
pub fn get_density_coefficients() -> Vec<DensityCoefficient>{
    DensityCoefficientService::get_density_coefficients()
}

#[logfn(Trace)]
#[tauri::command]
pub fn add_density_coefficient(density_coefficient: HashMap<String, Value>) -> DensityCoefficient{
    DensityCoefficientService::add_density_coefficient(density_coefficient)
}

#[tauri::command]
pub fn add_density_coefficients(file_path: &str) -> Vec<DensityCoefficient>{
    DensityCoefficientService::add_density_coefficients(file_path)
}