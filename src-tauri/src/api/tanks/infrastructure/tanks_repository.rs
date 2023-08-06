use diesel::query_dsl::methods::FilterDsl;
use diesel::{QueryDsl, SelectableHelper, RunQueryDsl, insert_into, ExpressionMethods};

use super::super::super::fms_core::establish_connection;
use super::models::{Tank, TankProfile, DensityCoefficient};
use super::schemas::tank::dsl::*;
use super::schemas::tank_profile::dsl::*;
use super::schemas::density_coefficient::dsl::*;


pub fn select_tanks(tank_vessel_id: String) -> Vec<Tank>{
    let connection = &mut establish_connection();
    let tanks = FilterDsl::filter(tank, vessel_id.eq(tank_vessel_id))
        .select(Tank::as_select())
        .load(connection).expect("Error during selecting tanks");
    tanks
}


pub fn insert_tank(new_tank: Tank) -> Tank{
    let connection = &mut establish_connection();
    
    let added_tank = insert_into(tank)
        .values(&new_tank)
        .get_result(connection).expect("Error during insert tank");
    added_tank
}


pub fn select_tank_profiles(profile_tank_id: String) -> Vec<TankProfile>{
    let connection = &mut establish_connection();
    let tank_profiles = FilterDsl::filter(tank_profile, tank_id.eq(profile_tank_id))
        .select(TankProfile::as_select())
        .load(connection).expect("Error during selecting tank profiles");
    tank_profiles
}


pub fn insert_tank_profile(new_tank_profile: TankProfile) -> TankProfile{
    let connection = &mut establish_connection();
    
    insert_into(tank_profile)
        .values(&new_tank_profile)
        .get_result(connection).expect("Error during insert tank profiles")
}


pub fn insert_tank_profiles(new_tank_profiles: Vec<TankProfile>) -> Vec<TankProfile>{
    let connection = &mut establish_connection();
    
    insert_into(tank_profile)
        .values(&new_tank_profiles)
        .get_results(connection).expect("Error during insert tank")
}


pub fn select_density_coefficients(density_temperature: f64) -> Vec<DensityCoefficient>{
    let connection = &mut establish_connection();
    let result = FilterDsl::filter(density_coefficient, temperature.eq(density_temperature))
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