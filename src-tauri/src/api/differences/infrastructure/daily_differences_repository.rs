use chrono::NaiveDate;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{insert_into, prelude::*, update};

use super::super::super::fms_core::establish_connection;
use super::super::domain::DailyDifference;
use super::daily_schema::dsl;

pub struct DailyDifferencesRepository;
impl DailyDifferencesRepository {
    pub fn insert_one(daily_difference: DailyDifference) -> DailyDifference {
        let connection = &mut establish_connection();

        let added_difference = insert_into(dsl::daily_difference)
            .values(&daily_difference)
            .get_result(connection)
            .expect("Error during insert difference");
        added_difference
    }

    pub fn select_all_in_range_of_dates(
        vessel_id: String,
        first_date: NaiveDate,
        last_date: NaiveDate,
    ) -> Vec<DailyDifference> {
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            dsl::daily_difference,
            dsl::vessel_id
                .eq(vessel_id)
                .and(dsl::date.ge(first_date).and(dsl::date.le(last_date))),
        )
        .order(dsl::date.desc())
        .select(DailyDifference::as_select())
        .get_results(connection)
        .expect("Error during selecting tanks");
        result
    }

    pub fn get_one_by_date_and_vessel(
        vessel_id: &String,
        today_date: NaiveDate,
    ) -> Option<DailyDifference> {
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            dsl::daily_difference,
            dsl::vessel_id.eq(vessel_id).and(dsl::date.eq(today_date)),
        )
        .select(DailyDifference::as_select())
        .get_result(connection)
        .optional()
        .unwrap();
        result
    }

    pub(crate) fn update(daily_difference: DailyDifference) {
        let connection = &mut establish_connection();
        update(FilterDsl::filter(
            dsl::daily_difference,
            dsl::date.eq(&daily_difference.date),
        ))
        .set(&daily_difference)
        .execute(connection)
        .expect("Error during update on daily difference table");
    }

    // TODO: Investigate issue with upsert
    // pub fn update(daily_difference: DailyDifference) {
    //     let connection = &mut establish_connection();
    //     let _ = insert_into(dsl::daily_difference)
    //     .values(&daily_difference)
    //     .on_conflict(dsl::date)
    //     .do_update()
    //     .set(
    //         &daily_difference
    //     ).execute(connection);
    // }
}
