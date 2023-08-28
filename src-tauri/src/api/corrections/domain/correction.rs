use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use super::super::super::fms_core::AbstractModel;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::super::infrastructure::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct TankCorrection{
    id: String,
    tank_id: String,
    trim: f64,
    correction: f64,
}
impl TankCorrection{
    pub fn new(data: HashMap<String, Value>) -> Self{
        Self{
            id: Uuid::new_v4().to_string(),
            tank_id: Self::parse_string(data.get("tank_id")),
            trim: Self::parse_f64(data.get("tank_trim")),
            correction: Self::parse_f64(data.get("height_correction")),
        }
    }
}
impl AbstractModel for TankCorrection{}