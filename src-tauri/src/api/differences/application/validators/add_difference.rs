use std::collections::HashMap;
use serde_json::Value;

use crate::api::fms_core::AbstractModel;


#[derive(Debug)]
pub struct AddDifference {
    pub tank_id: String,
    pub height: f64,
    pub trim: f64,
    pub temperature: f64,
    pub density_in_vacuum: f64,
    pub density_in_air: f64,
}
impl AddDifference {
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        Self {
            tank_id: Self::parse_string(data.get("tank_id")),
            height: Self::parse_f64(data.get("height")),
            trim: Self::parse_f64(data.get("trim")),
            temperature: Self::parse_f64(data.get("temperature")),
            density_in_vacuum: Self::parse_f64(data.get("density_in_vacuum")),
            density_in_air: Self::parse_f64(data.get("density_in_air")),
        }
    }
}
impl AbstractModel for AddDifference{}