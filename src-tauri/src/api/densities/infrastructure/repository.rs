use diesel::{insert_into, RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use diesel::query_dsl::methods::FilterDsl;

use super::{
    super::super::fms_core::establish_connection, 
    DensityCoefficient
};
use super::schema::dsl::*;


pub fn select_density_coefficients_for_temperature(density_temperature: f64) -> Vec<DensityCoefficient>{
    let connection = &mut establish_connection();
    let result = FilterDsl::filter(
        density_coefficient, temperature.eq(density_temperature)
    )
    .select(DensityCoefficient::as_select())
    .load(connection).expect("Error during selecting tank profiles");

    result
}


pub fn select_density_coefficients() -> Vec<DensityCoefficient>{
    let connection = &mut establish_connection();
    let result = density_coefficient
        .select(DensityCoefficient::as_select())
        .load(connection).expect("Error during selecting tank profiles");
    result
}


pub fn insert_density_coefficient(new_density_coefficient: DensityCoefficient) -> DensityCoefficient{
    let connection = &mut establish_connection();
    
    insert_into(density_coefficient)
        .values(&new_density_coefficient)
        .get_result(connection).expect("Error during insert tank profiles")
}


pub fn insert_density_coefficients(new_density_coefficients: Vec<DensityCoefficient>) -> Vec<DensityCoefficient>{
    let connection = &mut establish_connection();
    
    insert_into(density_coefficient)
        .values(&new_density_coefficients)
        .get_results(connection).expect("Error during insert tank")
}