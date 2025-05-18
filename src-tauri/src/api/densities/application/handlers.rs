use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::service::DensityCoefficientService;
use super::super::domain::DensityCoefficient;


#[tauri::command]
pub fn get_density_coefficients(search_form: HashMap<String, Value>) -> Vec<DensityCoefficient>{
    DensityCoefficientService::get_density_coefficients(search_form)
}

#[tauri::command]
pub fn get_density_coefficients_amount(search_form: HashMap<String, Value>) -> i64{
    DensityCoefficientService::get_density_coefficients_amount(search_form)
}

#[logfn(Trace)]
#[tauri::command]
pub fn add_density_coefficient(density_coefficient: HashMap<String, Value>) -> DensityCoefficient{
    DensityCoefficientService::add_density_coefficient(density_coefficient)
}

#[logfn(Trace)]
#[tauri::command]
pub fn add_density_coefficients(file_path: &str, factor: String) -> Vec<DensityCoefficient>{
    DensityCoefficientService::add_density_coefficients(file_path, factor)
}

#[logfn(Trace)]
#[tauri::command]
pub fn get_density_coefficient_in_vacuum(temperature: String, density: String) -> Option<DensityCoefficient>{
    let temperature = temperature.replace("\"", "").parse::<f64>().unwrap();
    let density = density.replace("\"", "").parse::<f64>().unwrap();
    DensityCoefficientService::get_density_coefficient_in_vacuum(temperature, density)
}


#[logfn(Trace)]
#[tauri::command]
pub fn get_density_coefficient_in_air(temperature: String, density: String) -> Option<DensityCoefficient>{
    let temperature = temperature.replace("\"", "").parse::<f64>().unwrap();
    let density = density.replace("\"", "").parse::<f64>().unwrap();
    DensityCoefficientService::get_density_coefficient_in_air(temperature, density)
}