use chrono::format::StrftimeItems;
use serde::{Deserialize, Serialize};

use crate::api::differences::infrastructure::Difference;

#[derive(Deserialize, Serialize, Debug)]
pub struct DifferenceDomain {
    pub id: String,
    pub tank_id: String,
    pub volume: f64,
    pub mass: f64,
    pub date_created: String,
    pub height: f64,
    pub trim: f64,
    pub temperature: f64,
    pub density: f64,
    pub fuel_type: String,
}
impl DifferenceDomain {
    fn from_difference(difference: Difference) -> Self {
        let date = difference.date_created;
        let fmt = StrftimeItems::new("%Y-%m-%d %H:%M");
        let date_time = date.format_with_items(fmt).to_string();

        Self {
            id: difference.id,
            tank_id: difference.tank_id,
            volume: difference.volume,
            mass: difference.mass,
            date_created: date_time,
            height: difference.height,
            trim: difference.trim,
            temperature: difference.temperature,
            density: difference.density,
            fuel_type: difference.fuel_type,
        }
    }

    pub fn from_differences(differences: Vec<Difference>) -> Vec<Self> {
        let mut difference_domains = vec![];

        for difference in differences {
            difference_domains.push(Self::from_difference(difference));
        }
        difference_domains
    }
}
