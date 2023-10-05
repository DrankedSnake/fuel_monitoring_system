use log_derive::logfn;

use crate::api::differences::{domain::DifferenceDomain, infrastructure::Difference};

use super::super::infrastructure::DifferencesRepository;


pub struct DifferencesService;
impl DifferencesService{
    #[logfn(Trace)]
    pub fn add_difference(difference: Difference) -> Difference {
        DifferencesRepository::insert_difference(difference)
    }

    #[logfn(Trace)]
    pub fn get_differences(tank_id: String) -> Vec<DifferenceDomain> {
        DifferenceDomain::from_differences(
            DifferencesRepository::select_differences(tank_id)
        )
    }
}