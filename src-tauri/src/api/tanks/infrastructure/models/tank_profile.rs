use std::{collections::HashMap};

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use csv::{self, Reader};

use crate::api::{AbstractModel};
use super::super::schemas::tank_profile;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = tank_profile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct TankProfile {
    id: String,
    tank_id: String,
    height: f64,
    trim: f64,
    volume: f64,
}
impl TankProfile {
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        Self {
            id: Uuid::new_v4().to_string(),
            tank_id: Self::parse_string(data.get("tank_id")),
            height: Self::parse_f64(data.get("tank_height")),
            trim: Self::parse_f64(data.get("tank_trim")),
            volume: Self::parse_f64(data.get("tank_volume")),
        }
    }

    pub fn is_value_empty(value: &String, empty_cell: &String) -> bool{
        if value == empty_cell{
            // println!("Empty volume in csv. Skipping pushing into collection.");
            return true;
        }
        else {
            return false
        }
    }

    pub fn from_csv_record(
        tank_id: &str, height: String, trim: String, volume: String, empty_cell: &String
    ) -> Option<Self>{
        let volume = volume.to_string();

        if !Self::is_value_empty(&volume, &empty_cell){
            return Some(Self {
                id: Uuid::new_v4().to_string(),
                tank_id: tank_id.to_string(),
                height: height.to_string().parse::<f64>().unwrap(),
                trim: trim.to_string().parse::<f64>().unwrap(),
                volume: volume.parse::<f64>().unwrap(),
            });
        }
        else{
            return None
        }
    }
}
impl AbstractModel for TankProfile{}