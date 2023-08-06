use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::api::AbstractModel;
use super::super::schemas::tank;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = tank)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Tank {
    pub id: String,
    pub name: String,
    pub available_volume: f64,
    pub current_volume: f64,
    pub vessel_id: String,
    pub previous_volume: f64,
}

impl Tank {
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        Self { 
            id: Uuid::new_v4().to_string(), 
            name: Self::parse_string(data.get("tank_name")), 
            available_volume: Self::parse_f64(data.get("available_volume")),
            current_volume: Self::parse_f64(data.get("current_volume")), 
            vessel_id: Self::parse_string(data.get("vessel_id")), 
            previous_volume: 0.0,
        }
    }
}

impl AbstractModel for Tank {

}