use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::super::domain::Vessel;
use super::service::VesselService;

#[tauri::command]
#[logfn(Trace)]
pub fn get_vessels() -> Vec<Vessel> {
    VesselService::get_vessels()
}

#[tauri::command]
#[logfn(Trace)]
pub fn add_vessel(vessel: HashMap<String, Value>) -> Vessel {
    VesselService::add_vessel(vessel)
}
