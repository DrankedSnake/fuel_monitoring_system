use serde::Serialize;

use crate::api::{fms_core::AbstractModel, tanks::infrastructure::Tank};

#[derive(Serialize, Debug)]

pub struct TankDomain {
    pub id: String,
    pub name: String,
    pub full_volume: f64,
    pub current_volume: f64,
    pub safe_volume: f64,
    pub vessel_id: String,
    pub previous_volume: f64,
    pub current_mass: f64,
    pub previous_mass: f64,
    pub fuel_type: String,
    pub tank_type: String,
    pub change_24_volume: f64,
    pub bunkering_volume: f64,
}
impl TankDomain {
    pub fn from_tank_model(tank: Tank) -> Self {
        Self {
            id: tank.id,
            name: tank.name,
            full_volume: tank.full_volume,
            current_volume: tank.current_volume,
            safe_volume: tank.safe_volume,
            current_mass: tank.current_mass,
            vessel_id: tank.vessel_id,
            previous_volume: tank.previous_volume,
            previous_mass: tank.previous_mass,
            fuel_type: tank.fuel_type,
            tank_type: tank.tank_type,
            change_24_volume: Self::round_f64(tank.previous_volume - tank.current_volume),
            bunkering_volume: Self::round_f64(tank.safe_volume - tank.current_volume),
        }
    }
}
impl AbstractModel for TankDomain {}
