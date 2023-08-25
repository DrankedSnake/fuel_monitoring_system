use log_derive::logfn;

use super::super::infrastructure::DifferencesRepository;
use super::super::domain::Difference;


pub struct DifferencesService;
impl DifferencesService{
    #[logfn(Trace)]
    pub fn add_difference(difference: Difference) -> Difference {
        DifferencesRepository::insert_difference(difference)
    }

    #[logfn(Trace)]
    pub fn get_differences(tank_id: String) -> Vec<Difference> {
        DifferencesRepository::select_differences(tank_id)
    }
}