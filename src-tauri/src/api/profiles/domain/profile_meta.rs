use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::api::fms_core::AbstractModel;

#[derive(Debug)]
pub struct ProfileMeta {
    pub tank_id: String,
    pub height: Option<f64>,
    pub trim: Option<f64>,
    pub offset: i64,
    pub limit: i64,
}
impl ProfileMeta {
    pub fn from_map(data: HashMap<String, Value>) -> Self {
        let pagination: &Map<String, Value> = Self::from_json_to_object(data.get("pagination"));
        let page = Self::from_number_to_i64(pagination.get("page"));
        let per_page = Self::from_number_to_i64(pagination.get("per_page"));

        let height = {
            let value = Self::parse_string(data.get("height"));
            if value != "" {
                Option::Some(value.parse::<f64>().unwrap())
            } else {
                Option::None
            }
        };
        let trim = {
            let value = Self::parse_string(data.get("trim"));
            if value != "" {
                Option::Some(value.parse::<f64>().unwrap())
            } else {
                Option::None
            }
        };

        Self {
            tank_id: Self::parse_string(data.get("tank_id")),
            height,
            trim,
            offset: (page - 1) * per_page,
            limit: per_page,
        }
    }
}
impl AbstractModel for ProfileMeta {}
