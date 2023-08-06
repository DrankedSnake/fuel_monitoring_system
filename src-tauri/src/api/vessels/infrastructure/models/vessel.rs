use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::api::AbstractModel;



#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::vessel_schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Vessel {
    pub id: String,
    pub name: String,
}
impl Vessel {
    pub fn from_map(data: HashMap<String, Value>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: Self::parse_string(data.get("vessel_name")),
        }
    }
}

impl AbstractModel for Vessel{}