use diesel::{QueryDsl, SelectableHelper, RunQueryDsl, insert_into};

use super::Vessel;
use super::super::super::fms_core::establish_connection;
use super::models::vessel_schema::dsl::*;

pub fn select_vessels() -> Vec<Vessel>{
    let connection = &mut establish_connection();
    let tanks = vessel
        .select(Vessel::as_select())
        .load(connection).expect("Error during selecting tanks");
    tanks
}

pub fn insert_vessel(new_tank: Vessel) -> Vessel{
    let connection = &mut establish_connection();
    
    let added_tank = insert_into(vessel)
        .values(&new_tank)
        .get_result(connection).expect("Error during insert tank");
    added_tank
}