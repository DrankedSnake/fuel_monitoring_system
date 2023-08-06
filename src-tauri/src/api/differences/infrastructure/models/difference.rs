use chrono::Datetime;

use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::api::AbstractModel;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::tank_schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Difference {
    id: String,
    from_tank_id: String,
    to_tank_id:String,
    tanker_id: String,
    consumer_id: String,
    difference_type: DifferenceType,
    fuel_volume: f64,
    fuel_mass: f64,
    density_coefficient_id: String,
    date_created: Datetime,
}

impl Difference {
    pub fn from_map(data: HashMap<String, Value>){
        Self {

        }
    }
}