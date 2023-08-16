use std::collections::HashMap;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use super::super::super::fms_core::AbstractModel;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Difference {
    pub id: String,
    pub tank_id: String,
    pub volume: f64,
    pub mass: f64,
    pub density_coefficient_id: String,
    pub date_created: DateTime<Utc>,
}

impl Difference {
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        Self {
            id: Uuid::new_v4().to_string(),
            tank_id: Self::parse_string(data.get("tank_id")),
            volume: Self::parse_f64(data.get("volume")),
            mass: Self::parse_f64(data.get("mass")),
            density_coefficient_id: Self::parse_string(data.get("density_coefficient_id")),
            date_created: Utc::now(),
        }
    }
}
impl AbstractModel for Difference{}