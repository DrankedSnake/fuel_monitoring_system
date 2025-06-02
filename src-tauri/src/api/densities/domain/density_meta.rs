use rust_decimal::prelude::ToPrimitive;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::api::fms_core::AbstractModel;

#[derive(Debug)]
pub struct DensityMeta {
    pub temperature: Option<f64>,
    pub density: Option<f64>,
    pub offset: i64,
    pub limit: i64,
}
impl DensityMeta {
    pub fn from_map(data: HashMap<String, Value>) -> Self {
        let pagination: &Map<String, Value> = Self::from_json_to_object(data.get("pagination"));
        let page = Self::from_number_to_i64(pagination.get("page"));
        let per_page = Self::from_number_to_i64(pagination.get("per_page"));

        let temperature = {
            let value = Self::parse_string(data.get("temperature"));
            if value != "" {
                Option::Some(value.parse::<f64>().unwrap())
            } else {
                Option::None
            }
        };
        let density = {
            let value = Self::parse_string(data.get("density"));
            if value != "" {
                Option::Some(value.parse::<f64>().unwrap())
            } else {
                Option::None
            }
        };

        Self {
            temperature,
            density,
            offset: (page - 1) * per_page,
            limit: per_page,
        }
    }
}
impl AbstractModel for DensityMeta {}
