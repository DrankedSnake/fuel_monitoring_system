use diesel::{insert_into, RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods, BoolExpressionMethods};
use diesel::query_dsl::methods::FilterDsl;
use log_derive::logfn;

use super::super::super::fms_core::establish_connection;
use super::schema::dsl::*;
use super::super::domain::DensityCoefficient;


pub struct DensityCoefficientsRepository;
impl DensityCoefficientsRepository {
    #[logfn(Trace)]
    pub fn select_density_coefficients_for_temperature(density_temperature: f64) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            density_coefficient, temperature.eq(density_temperature)
        )
        .select(DensityCoefficient::as_select())
        .load(connection).expect("Error during selecting density");
    
        result
    }
    
    pub fn select_density_coefficients() -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = density_coefficient
            .select(DensityCoefficient::as_select())
            .limit(15)
            .offset(0)
            .load(connection)
            .expect("Error during selecting density");
        result
    }
    
    #[logfn(Trace)]
    pub fn insert_density_coefficient(new_density_coefficient: DensityCoefficient) -> DensityCoefficient{
        let connection = &mut establish_connection();
        
        insert_into(density_coefficient)
            .values(&new_density_coefficient)
            .get_result(connection).expect("Error during insert density")
    }
    
    #[logfn(Trace)]
    pub fn insert_density_coefficients(new_density_coefficients: Vec<DensityCoefficient>) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        
        insert_into(density_coefficient)
            .values(&new_density_coefficients)
            .get_results(connection).expect("Error during insert density")
    }

    #[logfn(Trace)]
    pub fn select_density_coefficient(search_temperature: f64, search_density: f64) -> DensityCoefficient {
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            density_coefficient, temperature.eq(search_temperature).and(density.eq(search_density))
        )
        .select(DensityCoefficient::as_select())
        .get_result(connection).expect("Error during selecting density");
    
        result
    }
}
