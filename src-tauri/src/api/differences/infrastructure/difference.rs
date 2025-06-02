use chrono::{DateTime, Local};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::super::fms_core::AbstractModel;
use crate::api::profiles::domain::TankProfile;
use crate::api::{densities::domain::DensityCoefficient, tanks::Tank};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::super::infrastructure::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Difference {
    pub id: String,
    pub tank_id: String,
    pub volume: f64,
    pub mass: f64,
    pub date_created: DateTime<Local>,
    pub height: f64,
    pub trim: f64,
    pub temperature: f64,
    pub density: f64,
    pub fuel_type: String,
}

impl Difference {
    pub fn from_tank_density_and_profile(
        tank: &Tank,
        density_coefficient: &DensityCoefficient,
        profile: &TankProfile,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tank_id: tank.id.to_string(),
            volume: Self::round_f64(-1.0 * (tank.previous_volume - tank.current_volume)),
            mass: Self::round_f64(-1.0 * (tank.previous_mass - tank.current_mass)),
            date_created: Local::now(),
            height: profile.height,
            trim: profile.trim,
            temperature: density_coefficient.temperature,
            density: density_coefficient.density,
            fuel_type: tank.fuel_type.clone(),
        }
    }
}
impl AbstractModel for Difference {}
