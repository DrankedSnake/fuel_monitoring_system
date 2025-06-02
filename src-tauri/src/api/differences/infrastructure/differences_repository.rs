use diesel::query_dsl::methods::FilterDsl;
use diesel::{insert_into, prelude::*};
use log_derive::logfn;

use crate::api::differences::application::GetDifferences;

use super::super::super::fms_core::establish_connection;
use super::schema;
use super::Difference;

pub struct DifferencesRepository;
impl DifferencesRepository {
    #[logfn(Trace)]
    pub fn insert_difference(new_difference: Difference) -> Difference {
        let connection = &mut establish_connection();

        let added_difference = insert_into(schema::table)
            .values(&new_difference)
            .get_result(connection)
            .expect("Error during insert difference");

        added_difference
    }

    #[logfn(Trace)]
    pub fn select_differences(difference_meta: GetDifferences) -> Vec<Difference> {
        let connection = &mut establish_connection();
        let differences =
            FilterDsl::filter(schema::table, schema::tank_id.eq(difference_meta.tank_id))
                .order(schema::date_created.desc())
                .limit(difference_meta.limit)
                .offset(difference_meta.offset)
                .select(Difference::as_select())
                .load(connection)
                .expect("Error during selecting density");

        differences
    }

    #[logfn(Trace)]
    pub fn select_all_count(difference_meta: GetDifferences) -> i64 {
        let connection = &mut establish_connection();

        let result = FilterDsl::filter(schema::table, schema::tank_id.eq(difference_meta.tank_id))
            .count()
            .get_result(connection)
            .expect("Error during counting differences");
        result
    }
}
