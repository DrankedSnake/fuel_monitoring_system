use std::collections::HashMap;

use log_derive::logfn;
use serde_json::Value;
use crate::api::densities::domain::DensityMeta;

use super::super::infrastructure::DensityCoefficientsRepository;
use super::super::domain::DensityCoefficient;


pub struct DensityCoefficientService;

impl DensityCoefficientService {
    pub fn get_density_coefficients(search_form: HashMap<String, Value>) -> Vec<DensityCoefficient>{
        let density_meta = DensityMeta::from_map(search_form);

        if matches!(density_meta.temperature, Option::None) && matches!(density_meta.density, Option::None){
            return DensityCoefficientsRepository::select_all(density_meta)
        } else if matches!(density_meta.temperature, Option::Some(_f64)) && matches!(density_meta.density, Option::None) {
            return DensityCoefficientsRepository::select_all_by_temperature(density_meta)
        } else if matches!(density_meta.temperature, Option::None) && matches!(density_meta.density, Option::Some(_f64)) {
            return DensityCoefficientsRepository::select_all_by_density(density_meta)
        } else if matches!(density_meta.temperature, Option::Some(_f64)) && matches!(density_meta.density, Option::Some(_f64)) {
            return DensityCoefficientsRepository::select_all_by_temperature_and_density(density_meta)
        } else {
            panic!("Exceptional case raised!")
        }
    }

    pub fn get_density_coefficients_amount(search_form: HashMap<String, Value>) -> i64 {
        let density_meta = DensityMeta::from_map(search_form);

        if matches!(density_meta.temperature, Option::None) && matches!(density_meta.density, Option::None){
            return DensityCoefficientsRepository::select_all_count()
        } else if matches!(density_meta.temperature, Option::Some(_f64)) && matches!(density_meta.density, Option::None) {
            return DensityCoefficientsRepository::select_all_by_temperature_count(density_meta)
        } else if matches!(density_meta.temperature, Option::None) && matches!(density_meta.density, Option::Some(_f64)) {
            return DensityCoefficientsRepository::select_all_by_density_count(density_meta)
        } else if matches!(density_meta.temperature, Option::Some(_f64)) && matches!(density_meta.density, Option::Some(_f64)) {
            return DensityCoefficientsRepository::select_all_by_temperature_and_density_count(density_meta)
        } else {
            panic!("Exceptional case raised!")
        }
    }

    #[logfn(Trace)]
    pub fn add_density_coefficient(data: HashMap<String, Value>) -> DensityCoefficient{
        DensityCoefficientsRepository::insert_one(
            DensityCoefficient::from_map(data)
        )
    }

    #[logfn(Trace)]
    pub fn add_density_coefficients(file_path: &str, factor: String) -> Vec<DensityCoefficient>{
        DensityCoefficientsRepository::insert_many(
            DensityCoefficient::from_csv(file_path, factor)
        )
    }

    #[logfn(Trace)]
    pub fn get_density_coefficient_in_vacuum(temperature: f64, density: f64) -> Option<DensityCoefficient> {
        DensityCoefficientsRepository::select_one_for_vacuum(
            temperature, density
        )
    }

    #[logfn(Trace)]
    pub fn get_density_coefficient_in_air(temperature: f64, density: f64) -> Option<DensityCoefficient> {
        DensityCoefficientsRepository::select_one_for_air(
            temperature, density
        )
    }
}