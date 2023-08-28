use log_derive::logfn;

use super::super::domain::TankCorrection;
use super::TankCorrectionService;


#[logfn(Trace)]
#[tauri::command]
pub fn get_tank_corrections(tank_id: String) -> Vec<TankCorrection>{
    TankCorrectionService::get_tank_corrections(tank_id)
}