use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::super::infrastructure::VesselsRepository;
use super::super::domain::Vessel;


pub struct VesselService;
impl VesselService {
    #[logfn(Trace)]
    pub fn get_vessels() -> Vec<Vessel>{
        VesselsRepository::select_vessels()        
    }

    #[logfn(Trace)]
    pub fn add_vessel(data: HashMap<String, Value>) -> Vessel{
        VesselsRepository::insert_vessel(
            Vessel::from_map(data)
        )
    }
    
}