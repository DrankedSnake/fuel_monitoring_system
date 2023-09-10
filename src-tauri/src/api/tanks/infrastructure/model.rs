use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use super::super::super::fms_core::AbstractModel;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Tank {
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
}

impl Tank {
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        let mut safe_volume = Self::parse_f64(data.get("safe_volume"));

        if safe_volume > 1.0{
            safe_volume = Self::round_f64(safe_volume / 100.0);
        }

        Self { 
            id: Uuid::new_v4().to_string(), 
            name: Self::parse_string(data.get("tank_name")), 
            full_volume: Self::parse_f64(data.get("full_volume")),
            current_volume: Self::parse_f64(data.get("current_volume")), 
            safe_volume: Self::round_f64(Self::parse_f64(data.get("full_volume")) * safe_volume), 
            current_mass: Self::parse_f64(data.get("current_mass")),
            vessel_id: Self::parse_string(data.get("vessel_id")),
            previous_volume: Self::parse_f64(data.get("previous_volume")),
            previous_mass: Self::parse_f64(data.get("previous_mass")),
            fuel_type: Self::parse_string(data.get("fuel_type")),
            tank_type: Self::parse_string(data.get("tank_type")),
        }
    }

    pub fn update(&mut self, volume: f64, density: f64, coefficient: f64){
        self.previous_volume = self.current_volume;
        self.previous_mass = self.current_mass;
        self.current_volume = volume;
        self.current_mass = Self::round_f64(volume * density * coefficient);
    }
}

impl AbstractModel for Tank {

}