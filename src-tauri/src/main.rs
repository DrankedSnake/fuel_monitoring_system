// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod api;

use std::{path::PathBuf, str::FromStr};

use api::{
    get_tanks, 
    add_tank,
    get_vessels,
    add_vessel,
    get_tank_profiles,
    get_tank_profile,
    add_tank_profile,
    add_tank_profiles,
    add_density_coefficient,
    add_density_coefficients,
    get_density_coefficients,
    add_difference,
    get_differences,
    get_daily_differences_for_current_month,
    get_tankers,
    get_tank_corrections,
};
use tauri_plugin_log::LogTarget;


fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default().target(
                LogTarget::Folder(PathBuf::from_str("/home/nikita/.fms/logs").unwrap())
            ).build())
        .invoke_handler(tauri::generate_handler![
            get_tanks,
            add_tank,
            get_vessels, 
            add_vessel,
            get_tank_profiles,
            get_tank_profile,
            add_tank_profile,
            add_tank_profiles,
            add_density_coefficient,
            add_density_coefficients,
            get_density_coefficients,
            add_difference,
            get_differences,
            get_daily_differences_for_current_month,
            get_tankers,
            get_tank_corrections,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}