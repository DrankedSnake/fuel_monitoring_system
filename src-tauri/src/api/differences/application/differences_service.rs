use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use crate::api::differences::application::GetDifferences;

use super::super::domain::DifferenceDomain;
use super::super::infrastructure::Difference;
use super::super::infrastructure::DifferencesRepository;


pub struct DifferencesService;
impl DifferencesService{
    #[logfn(Trace)]
    pub fn add_difference(difference: Difference) -> Difference {
        DifferencesRepository::insert_difference(difference)
    }

    #[logfn(Trace)]
    pub fn get_differences(search_form: HashMap<String, Value>) -> Vec<DifferenceDomain> {
        let difference_meta = GetDifferences::from_map(search_form);
        DifferenceDomain::from_differences(
            DifferencesRepository::select_differences(difference_meta)
        )
    }

    #[logfn(Trace)]
    pub fn get_difference_amount(search_form: HashMap<String, Value>) -> i64 {
        let difference_meta = GetDifferences::from_map(search_form);
        DifferencesRepository::select_all_count(difference_meta)
    }
}