use super::super::infrastructure::TankCorrectionsRepository;
use super::super::domain::TankCorrection;


pub struct TankCorrectionService;
impl TankCorrectionService{
    pub fn get_tank_corrections(tank_id: String) -> Vec<TankCorrection>{
        TankCorrectionsRepository::select_tank_corrections(tank_id)
    }
}