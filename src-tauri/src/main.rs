#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tanks;

use tanks::get_tanks;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tanks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
