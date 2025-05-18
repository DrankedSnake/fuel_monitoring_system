use std::collections::HashMap;
use serde_json::{Value, Map};

use crate::api::fms_core::AbstractModel;


#[derive(Debug)]
pub struct GetDifferences {
    pub tank_id: String,
    pub offset: i64,
    pub limit: i64,
}
impl GetDifferences {
    pub fn from_map(data: HashMap<String, Value>) -> Self{
        let pagination: &Map<String, Value> = Self::from_json_to_object(data.get("pagination"));
        let page = Self::from_number_to_i64(pagination.get("page"));
        let per_page = Self::from_number_to_i64(pagination.get("per_page"));

        Self {
            tank_id: Self::parse_string(data.get("tank_id")),
            offset: (page - 1) * per_page,
            limit: per_page,
        }
    }
}
impl AbstractModel for GetDifferences{}