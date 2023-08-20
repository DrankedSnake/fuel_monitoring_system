use super::differences_service::DifferencesService;
use super::daily_differences_service::DailyDifferencesService;
use crate::api::tanks::application::service::TankService;
use crate::api::profiles::application::service::TankProfileService;
use crate::api::densities::application::service::DensityCoefficientService;

use super::super::domain::{
    Difference,
    DailyDifference,
};


#[tauri::command]
pub fn add_difference(tank_id: String, tank_height: String, tank_trim: String, temperature: String, density: String) -> Difference{
    let mut tank = TankService::get_tank_by_id(tank_id);
    let tank_profile = TankProfileService::get_tank_profile_by_height_and_trim(
        tank.id.clone(), 
        tank_height.replace("\"", "").parse::<f64>().unwrap(),
        tank_trim.replace("\"", "").parse::<f64>().unwrap(),
    );
    let density_coefficient = DensityCoefficientService::get_density_coefficient(
        temperature.replace("\"", "").parse::<f64>().unwrap(),
        density.replace("\"", "").parse::<f64>().unwrap(),
    );
    tank.update(tank_profile.volume, density_coefficient.density, density_coefficient.coefficient);
    tank = TankService::update_tank(tank);

    let difference = DifferencesService::add_difference(
        Difference::from_tank_and_density(&tank, &density_coefficient)
    );
    let result = DailyDifferencesService::get_today_difference(&tank.vessel_id);

    if let Some(mut value) = result {
        value.update(&difference);
        DailyDifferencesService::update_difference(value);
    }else {
        DailyDifferencesService::add_daily_difference(
            DailyDifference::from_tank_and_difference(&tank, &difference)
        );
    }
    

    difference
}

#[tauri::command]
pub fn get_differences(tank_id: String) -> Vec<Difference>{
    DifferencesService::get_differences(tank_id)
}

#[tauri::command]
pub fn get_daily_differences_for_current_month(vessel_id: String) -> Vec<DailyDifference>{
    DailyDifferencesService::get_daily_differences_for_current_month(vessel_id)
}