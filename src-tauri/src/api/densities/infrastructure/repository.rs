use diesel::{prelude::*, insert_into};
use diesel::query_dsl::methods::FilterDsl;
use log_derive::{logfn, logfn_inputs};

use super::super::super::fms_core::establish_connection;
use super::schema;
use super::super::domain::DensityCoefficient;


pub struct DensityCoefficientsRepository;
impl DensityCoefficientsRepository {
    #[logfn(Trace)]
    pub fn select_density_coefficients_for_temperature(temperature: f64) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            schema::table, 
            schema::temperature.eq(temperature)
        )
        .select(DensityCoefficient::as_select())
        .load(connection).expect("Error during selecting density");
    
        result
    }
    
    pub fn select_density_coefficients() -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = schema::table
            .select(DensityCoefficient::as_select())
            .limit(15)
            .offset(0)
            .load(connection)
            .expect("Error during selecting density");
        result
    }
    
    #[logfn(Trace)]
    pub fn insert_density_coefficient(density_coefficient: DensityCoefficient) -> DensityCoefficient{
        let connection = &mut establish_connection();
        
        insert_into(schema::table)
            .values(&density_coefficient)
            .get_result(connection).expect("Error during insert density")
    }
    
    #[logfn(Trace)]
    pub fn insert_density_coefficients(density_coefficients: Vec<DensityCoefficient>) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        
        insert_into(schema::table)
            .values(&density_coefficients)
            .get_results(connection).expect("Error during insert density")
    }

    #[logfn_inputs(Trace)]
    pub fn select_density_coefficient(temperature: f64, density: f64) -> Option<DensityCoefficient> {
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            schema::table, 
            schema::temperature.eq(temperature)
            .and(schema::density.eq(density))
        )
        .select(DensityCoefficient::as_select())
        .get_result(connection).optional().unwrap();
    
        result
    }
}
