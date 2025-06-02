// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod api;

use api::{
    add_density_coefficient, add_density_coefficients, add_difference, add_tank, add_tank_profile,
    add_tank_profiles, add_vessel, get_daily_differences_for_current_month,
    get_density_coefficient_in_air, get_density_coefficient_in_vacuum, get_density_coefficients,
    get_density_coefficients_amount, get_differences, get_differences_amount, get_tank_profile,
    get_tank_profiles, get_tank_profiles_amount, get_tankers, get_tanks, get_vessels,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("fms_logs".to_string()),
                    },
                ))
                .build(),
        )
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
            get_differences_amount,
            get_tankers,
            get_tank_profiles_amount,
            get_density_coefficient_in_air,
            get_density_coefficient_in_vacuum,
            get_density_coefficients_amount,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
