use diesel::{insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use log_derive::logfn;

use super::super::super::fms_core::establish_connection;
use super::super::domain::Vessel;
use super::schema::dsl::*;

pub struct VesselsRepository;
impl VesselsRepository {
    #[logfn(Trace)]
    pub fn select_vessels() -> Vec<Vessel> {
        let connection = &mut establish_connection();
        let vessels = vessel
            .select(Vessel::as_select())
            .load(connection)
            .expect("Erorr during select vessels");
        vessels
    }

    #[logfn(Trace)]
    pub fn insert_vessel(new_tank: Vessel) -> Vessel {
        let connection = &mut establish_connection();

        let added_tank = insert_into(vessel)
            .values(&new_tank)
            .get_result(connection)
            .expect("Error during insert tank");
        added_tank
    }
}
