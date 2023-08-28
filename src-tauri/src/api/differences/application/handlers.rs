use log_derive::logfn;

use super::differences_service::DifferencesService;
use super::daily_differences_service::DailyDifferencesService;
use crate::api::tanks::application::service::TankService;
use crate::api::profiles::application::service::TankProfileService;
use crate::api::densities::application::service::DensityCoefficientService;

use super::super::domain::{
    Difference,
    DailyDifference,
};


#[logfn(Trace)]
#[tauri::command]
pub fn add_difference(tank_id: String, tank_height: String, tank_trim: String, temperature: String, density: String) -> Difference{
    let mut tank = TankService::get_tank_by_id(tank_id);

    // TODO: check correction type of tank
    //   1. if correction type trim get profile like always
    //   2. if correction type height get correction first and calculate correction height
    //   3. provide another method which will get profile by tank_id and corrected height
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

#[logfn(Trace)]
#[tauri::command]
pub fn get_differences(tank_id: String) -> Vec<Difference>{
    DifferencesService::get_differences(tank_id)
}

#[logfn(Trace)]
#[tauri::command]
pub fn get_daily_differences_for_current_month(vessel_id: String) -> Vec<DailyDifference>{
    DailyDifferencesService::get_daily_differences_for_current_month(vessel_id)
}