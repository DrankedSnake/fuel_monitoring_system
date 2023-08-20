use std::collections::HashMap;

use chrono::{DateTime, Utc, Local};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::api::{
    densities::domain::DensityCoefficient, 
    tanks::domain::Tank
};
use super::super::super::fms_core::AbstractModel;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::super::infrastructure::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Difference {
    pub id: String,
    pub tank_id: String,
    pub volume: f64,
    pub mass: f64,
    pub density_coefficient_id: String,
    pub date_created: DateTime<Local>,
}

impl Difference {
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        Self {
            id: Uuid::new_v4().to_string(),
            tank_id: Self::parse_string(data.get("tank_id")),
            volume: Self::parse_f64(data.get("volume")),
            mass: Self::parse_f64(data.get("mass")),
            density_coefficient_id: Self::parse_string(data.get("density_coefficient_id")),
            date_created: Local::now(),
        }
    }

    pub fn from_tank_and_density(tank: &Tank, density_coefficient: &DensityCoefficient) -> Self{
        Self {
            id: Uuid::new_v4().to_string(),
            tank_id: tank.id.to_string(),
            volume: tank.previous_volume - tank.current_volume,
            mass: tank.previous_mass - tank.current_mass,
            density_coefficient_id: density_coefficient.id.to_string(),
            date_created: Local::now(),
        }
    }
}
impl AbstractModel for Difference{}