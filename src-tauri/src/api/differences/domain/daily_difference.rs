use chrono::{Datelike, Local, NaiveDate};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::differences::infrastructure::Difference;
use crate::api::fms_core::AbstractModel;

use crate::api::tanks::Tank;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = super::super::infrastructure::daily_schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug)]
pub struct DailyDifference {
    pub id: String,
    pub vessel_id: String,
    pub hfo_volume: f64,
    pub hfo_mass: f64,
    pub mgo_volume: f64,
    pub mgo_mass: f64,
    pub date: NaiveDate,
}
impl DailyDifference {
    pub fn update(&mut self, difference: &Difference) {
        if difference.fuel_type == "HFO".to_string() {
            self.hfo_volume = Self::round_f64(self.hfo_volume + difference.volume);
            self.hfo_mass = Self::round_f64(self.hfo_mass + difference.mass);
        } else {
            self.mgo_volume = Self::round_f64(self.mgo_volume + difference.volume);
            self.mgo_mass = Self::round_f64(self.mgo_mass + difference.mass);
        }
    }

    pub fn from_tank_and_difference(tank: &Tank, difference: &Difference) -> Self {
        let date = Local::now();
        if tank.fuel_type == "HFO".to_string() {
            Self {
                id: Uuid::new_v4().to_string(),
                vessel_id: tank.vessel_id.to_string(),
                hfo_volume: difference.volume,
                hfo_mass: difference.mass,
                mgo_volume: 0.0,
                mgo_mass: 0.0,
                date: NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap(),
            }
        } else {
            Self {
                id: Uuid::new_v4().to_string(),
                vessel_id: tank.vessel_id.to_string(),
                mgo_volume: difference.volume,
                mgo_mass: difference.mass,
                hfo_volume: 0.0,
                hfo_mass: 0.0,
                date: NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap(),
            }
        }
    }
}
impl AbstractModel for DailyDifference {}
