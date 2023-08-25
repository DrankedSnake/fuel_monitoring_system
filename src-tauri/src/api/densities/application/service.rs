use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;

use super::super::infrastructure::DensityCoefficientsRepository;
use super::super::domain::DensityCoefficient;


pub struct DensityCoefficientService;
impl DensityCoefficientService {
    pub fn get_density_coefficients() -> Vec<DensityCoefficient>{
        DensityCoefficientsRepository::select_density_coefficients()
    }

    #[logfn(Trace)]
    pub fn add_density_coefficient(data: HashMap<String, Value>) -> DensityCoefficient{
        DensityCoefficientsRepository::insert_density_coefficient(
            DensityCoefficient::from_map(data)
        )
    }

    #[logfn(Trace)]
    pub fn add_density_coefficients(file_path: &str) -> Vec<DensityCoefficient>{
        DensityCoefficientsRepository::insert_density_coefficients(
            DensityCoefficient::from_csv(file_path)
        )
    }

    #[logfn(Trace)]
    pub(crate) fn get_density_coefficient(temperature: f64, density: f64) -> DensityCoefficient {
        DensityCoefficientsRepository::select_density_coefficient(
            temperature, density
        )
    }
}