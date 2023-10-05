use diesel::{prelude::*, insert_into};
use diesel::query_dsl::methods::FilterDsl;
use log_derive::logfn;

use super::Difference;
use super::schema;
use super::super::super::fms_core::establish_connection;


pub struct DifferencesRepository;
impl DifferencesRepository{
    #[logfn(Trace)]
    pub fn insert_difference(new_difference: Difference) -> Difference{
        let connection = &mut establish_connection();
        
        let added_difference = insert_into(schema::table)
        .values(&new_difference)
        .get_result(connection).expect("Error during insert difference");

        added_difference
    }

    #[logfn(Trace)]
    pub fn select_differences(difference_tank_id: String) -> Vec<Difference>{
        let connection = &mut establish_connection();
        let differences = FilterDsl::filter(
            schema::table, 
            schema::tank_id.eq(difference_tank_id)
        )
        .select(Difference::as_select())
        .order(schema::date_created.desc())
        .load(connection).expect("Error during selecting tanks");

        differences
    }
}