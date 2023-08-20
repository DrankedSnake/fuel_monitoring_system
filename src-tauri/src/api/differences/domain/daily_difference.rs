use chrono::{NaiveDate, Local, Datelike};
use diesel::{Queryable, Selectable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::api::fms_core::AbstractModel;

use super::Difference;
use crate::api::tanks::domain::Tank;


#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = super::super::infrastructure::daily_schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug)]
pub struct DailyDifference{
    pub id: String,
    pub vessel_id: String,
    pub volume: f64,
    pub mass: f64,
    pub date: NaiveDate,
}
impl DailyDifference {
    pub fn update(&mut self, difference: &Difference){
        self.volume += difference.volume;
        self.mass += difference.mass;     
    }

    pub fn from_tank_and_difference(tank: &Tank, difference: &Difference) -> Self{
        let date = Local::now();
        Self { 
            id: Uuid::new_v4().to_string(),
            vessel_id: tank.vessel_id.to_string(),
            volume: difference.volume, 
            mass: difference.mass, 
            date: NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap()
        }
    }
}
impl AbstractModel for DailyDifference {}