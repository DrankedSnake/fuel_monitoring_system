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
pub struct Vessel {
    pub id: String,
    pub name: String,
    pub year: i32,
    pub dead_weight: i32,
    pub imo: String,
    pub company: String,
}
impl Vessel {
    pub fn from_map(data: HashMap<String, Value>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: Self::parse_string(data.get("vessel_name")),
            year: Self::parse_i32(data.get("vessel_year")),
            dead_weight: Self::parse_i32(data.get("vessel_dead_weight")),
            imo: Self::parse_string(data.get("imo")),
            company: Self::parse_string(data.get("company")),
        }
    }
}

impl AbstractModel for Vessel{}