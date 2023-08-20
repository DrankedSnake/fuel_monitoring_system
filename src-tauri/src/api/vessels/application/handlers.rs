use std::collections::HashMap;

use serde_json::Value;

use super::service::VesselService;
use super::super::domain::Vessel;


#[tauri::command]
pub fn get_vessels() -> Vec<Vessel>{
    VesselService::get_vessels()
}


#[tauri::command]
pub fn add_vessel(vessel: HashMap<String, Value>) -> Vessel{
    VesselService::add_vessel(vessel)
}