use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::api::AbstractModel;
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
            height: Self::parse_f64(data.get("hight")),
            trim: Self::parse_f64(data.get("trim")),
            volume: Self::parse_f64(data.get("volume")),
        }
    }
}
impl AbstractModel for TankProfile{}