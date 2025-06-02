use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::daily_differences_service::DailyDifferencesService;
use super::differences_service::DifferencesService;
use crate::api::densities::application::service::DensityCoefficientService;
use crate::api::differences::application::validators::AddDifference;
use crate::api::differences::infrastructure::Difference;
use crate::api::profiles::application::service::TankProfileService;
use crate::api::{differences::domain::DifferenceDomain, tanks::application::service::TankService};

use super::super::domain::DailyDifference;

#[logfn(Trace)]
#[tauri::command]
pub fn add_difference(form: HashMap<String, Value>) -> Difference {
    let form = AddDifference::from_map(form);
    let mut tank = TankService::get_tank_by_id(form.tank_id);

    let tank_profile = TankProfileService::get_tank_profile(tank.id.clone(), form.height, form.trim).unwrap();
    let density_coefficient_in_vacuum =
        DensityCoefficientService::get_density_coefficient_in_vacuum(form.temperature, form.density_in_vacuum).unwrap();
    let density_coefficient_in_air =
        DensityCoefficientService::get_density_coefficient_in_air(15.0, form.density_in_air).unwrap();
    tank.update(
        &tank_profile,
        &density_coefficient_in_vacuum,
        &density_coefficient_in_air,
    );
    tank = TankService::update_tank(tank);

    let difference = DifferencesService::add_difference(Difference::from_tank_density_and_profile(
        &tank,
        &density_coefficient_in_vacuum,
        &tank_profile,
    ));
    let result = DailyDifferencesService::get_today_difference(&tank.vessel_id);

    if let Some(mut value) = result {
        value.update(&difference);
        DailyDifferencesService::update_difference(value);
    } else {
        DailyDifferencesService::add_daily_difference(DailyDifference::from_tank_and_difference(&tank, &difference));
    }

    difference
}

#[logfn(Trace)]
#[tauri::command]
pub fn get_differences(search_form: HashMap<String, Value>) -> Vec<DifferenceDomain> {
    DifferencesService::get_differences(search_form)
}

#[logfn(Trace)]
#[tauri::command]
pub fn get_differences_amount(search_form: HashMap<String, Value>) -> i64 {
    DifferencesService::get_difference_amount(search_form)
}

#[logfn(Trace)]
#[tauri::command]
pub fn get_daily_differences_for_current_month(vessel_id: String, date: String) -> Vec<DailyDifference> {
    DailyDifferencesService::get_daily_differences(vessel_id, date)
}
