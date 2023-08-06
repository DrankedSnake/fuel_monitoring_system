use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::api::AbstractModel;
use super::super::schemas::density_coefficient;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = density_coefficient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct DensityCoefficient{
    id: String,
    temperature: f64,
    density: f64,
    coefficient: f64,
}
impl DensityCoefficient{
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        Self{
            id: Uuid::new_v4().to_string(),
            temperature: Self::parse_f64(data.get("temperature")),
            density: Self::parse_f64(data.get("density")),
            coefficient: Self::parse_f64(data.get("coefficient")),      
        }
    }
}
impl AbstractModel for DensityCoefficient {}