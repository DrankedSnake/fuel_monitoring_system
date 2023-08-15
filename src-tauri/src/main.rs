// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod api;

use api::{
    get_tanks, 
    add_tank,
    get_vessels,
    add_vessel,
    get_tank_profiles,
    add_tank_profile,
    add_tank_profiles,
    add_density_coefficient,
    add_density_coefficients,
    get_density_coefficients,
};


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tanks,
            add_tank,
            get_vessels, 
            add_vessel,
            get_tank_profiles,
            add_tank_profile,
            add_tank_profiles,
            add_density_coefficient,
            add_density_coefficients,
            get_density_coefficients,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Only for debug
    // use tauri::Manager;
    // tauri::Builder::default()
    // .invoke_handler(
    //     tauri::generate_handler![
    //         get_tanks,
    //         add_tank,
    //         get_vessels, 
    //         add_vessel,
    //         get_tank_profiles,
    //         add_tank_profile,
    //         add_tank_profiles,
    //         add_density_coefficient,
    //         add_density_coefficients,
    //         get_density_coefficients,
    //         ]
    //     ).setup(
    //         |app| {
    //             #[cfg(debug_assertions)]
    //             {
    //                 let window = app.get_window("main").unwrap();
    //                 window.open_devtools();
    //                 window.close_devtools();
    //             }
    //             Ok(())
    //         }
    //     ).run(tauri::generate_context!()).expect("error while running tauri application");
}
// 