use chrono::{DateTime, Local, NaiveDate, Datelike};

use super::super::infrastructure::DailyDifferencesRepository;
use super::super::domain::DailyDifference;


pub struct DailyDifferencesService;
impl DailyDifferencesService{
    pub fn get_daily_differences_for_current_month(vessel_id: String) -> Vec<DailyDifference>{
        let local: DateTime<Local> = Local::now();
        let first_date = NaiveDate::from_ymd_opt(local.year(), local.month(), 1).unwrap();

        let last_date = {
            if local.month() < 12 {
                NaiveDate::from_ymd_opt(local.year(), local.month() + 1, 1).unwrap().pred_opt().unwrap()
            } else {
                NaiveDate::from_ymd_opt(local.year() + 1, 1, 1).unwrap().pred_opt().unwrap()
            }
        };

        DailyDifferencesRepository::select_all_in_range_of_dates(vessel_id, first_date, last_date)
    }

    pub fn get_today_difference(vessel_id: &String) -> Option<DailyDifference> {
        let local: DateTime<Local> = Local::now();
        let today_date = NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap();
        DailyDifferencesRepository::get_one_by_date_and_vessel(vessel_id, today_date)
    }

    pub fn update_difference(daily_difference: DailyDifference) {
        DailyDifferencesRepository::update(daily_difference)
    }

    pub fn add_daily_difference(daily_difference: DailyDifference) -> DailyDifference{
        DailyDifferencesRepository::insert_one(daily_difference)
    }
}