use super::super::infrastructure::DifferencesRepository;
use super::super::domain::Difference;


pub struct DifferencesService;
impl DifferencesService{
    pub(crate) fn add_difference(difference: Difference) -> Difference {
        DifferencesRepository::insert_difference(difference)
    }

    pub fn get_differences(tank_id: String) -> Vec<Difference> {
        DifferencesRepository::select_differences(tank_id)
    }
}