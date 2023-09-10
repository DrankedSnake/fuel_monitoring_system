use std::collections::HashMap;

use csv::Reader;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use super::super::super::fms_core::AbstractModel;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::super::infrastructure::schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct DensityCoefficient{
    pub id: String,
    pub temperature: f64,
    pub density: f64,
    pub coefficient: f64,
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

    pub fn from_csv(file_path: &str) -> Vec<Self>{
        let mut items: Vec<Self> = Vec::new();

        let mut reader = Reader::from_path(file_path).expect("No such file found.");
        let headers = reader.headers().unwrap().clone();
        let records = reader.records();

        for record in records {
            let record = record.unwrap();
            let mut index = 1;
            
            while index < record.len(){
                items.push(
                    Self{
                        id: Uuid::new_v4().to_string(),
                        temperature: record[0].to_string().parse::<f64>().unwrap(),
                        density: headers[index].to_string().parse::<f64>().unwrap(),
                        coefficient: record[index].to_string().parse::<f64>().unwrap(),
                    }
                );
                index += 1;
            }
        }
        items

    }
}
impl AbstractModel for DensityCoefficient {}