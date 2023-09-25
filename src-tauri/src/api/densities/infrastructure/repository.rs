use diesel::{prelude::*, insert_into};
use diesel::query_dsl::methods::FilterDsl;
use log_derive::{logfn, logfn_inputs};
use crate::api::densities::domain::DensityMeta;

use super::super::super::fms_core::establish_connection;
use super::schema;
use super::super::domain::DensityCoefficient;


pub struct DensityCoefficientsRepository;

impl DensityCoefficientsRepository {
    #[logfn(Trace)]
    pub fn select_all_by_temperature_count(density_meta: DensityMeta) -> i64{
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            schema::table, 
            schema::temperature.eq(density_meta.temperature.unwrap())
        )
        .count()
        .get_result(connection).expect("Error during selecting density");
    
        result
    }

    #[logfn(Trace)]
    pub fn select_all_by_temperature(density_meta: DensityMeta) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            schema::table, 
            schema::temperature.eq(density_meta.temperature.unwrap())
        )
        .limit(density_meta.limit)
        .offset(density_meta.offset)
        .select(DensityCoefficient::as_select())
        .load(connection).expect("Error during selecting density");
    
        result
    }

    #[logfn(Trace)]
    pub fn select_all_by_density_count(density_meta: DensityMeta) -> i64{
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            schema::table, 
            schema::density.eq(density_meta.density.unwrap())
        )
        .count()
        .get_result(connection).expect("Error during selecting density");
    
        result
    }

    #[logfn(Trace)]
    pub fn select_all_by_density(density_meta: DensityMeta) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            schema::table, 
            schema::density.eq(density_meta.density.unwrap())
        )
        .limit(density_meta.limit)
        .offset(density_meta.offset)
        .select(DensityCoefficient::as_select())
        .load(connection).expect("Error during selecting density");
    
        result
    }

    #[logfn(Trace)]
    pub fn select_all_by_temperature_and_density_count(density_meta: DensityMeta) -> i64 {
        let connection = &mut establish_connection();

        let result = FilterDsl::filter(
            schema::table, 
            schema::temperature.eq(density_meta.temperature.unwrap())
            .and(schema::density.eq(density_meta.density.unwrap()))
        )
        .count()
        .get_result(connection).expect("Error during counting profiles");
        result
    }

    #[logfn(Trace)]
    pub fn select_all_by_temperature_and_density(density_meta: DensityMeta) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(
            schema::table, 
            schema::temperature.eq(density_meta.temperature.unwrap())
            .and(schema::density.eq(density_meta.density.unwrap()))
        )
        .limit(density_meta.limit)
        .offset(density_meta.offset)
        .select(DensityCoefficient::as_select())
        .load(connection).expect("Error during selecting density");
    
        result
    }

    #[logfn(Trace)]
    pub fn select_all_count(density_meta: DensityMeta) -> i64 {
        let connection = &mut establish_connection();

        let result = schema::table.select(schema::id)
            .count()
            .get_result(connection).expect("Error during counting profiles");
        result
    }
    
    #[logfn(Trace)]
    pub fn select_all(density_meta: DensityMeta) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        let result = schema::table
            .select(DensityCoefficient::as_select())
            .limit(density_meta.limit)
            .offset(density_meta.offset)
            .load(connection)
            .expect("Error during selecting density");
        result
    }
    
    #[logfn(Trace)]
    pub fn insert_one(density_coefficient: DensityCoefficient) -> DensityCoefficient{
        let connection = &mut establish_connection();
        
        insert_into(schema::table)
            .values(&density_coefficient)
            .get_result(connection).expect("Error during insert density")
    }
    
    #[logfn(Trace)]
    pub fn insert_many(density_coefficients: Vec<DensityCoefficient>) -> Vec<DensityCoefficient>{
        let connection = &mut establish_connection();
        
        insert_into(schema::table)
            .values(&density_coefficients)
            .get_results(connection).expect("Error during insert density")
    }

    #[logfn_inputs(Trace)]
    pub fn select_one_by(temperature: f64, density: f64) -> Option<DensityCoefficient> {
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
