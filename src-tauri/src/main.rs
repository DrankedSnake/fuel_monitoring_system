// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod api;

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
    get_tankers,
};


fn main() {
    tauri::Builder::default()
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
            get_tankers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}