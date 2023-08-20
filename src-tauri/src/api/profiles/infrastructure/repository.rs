use diesel::{prelude::*, insert_into};
use diesel::query_dsl::methods::FilterDsl;

use super::super::super::fms_core::establish_connection;
use super::schema::dsl::*;
use super::super::domain::TankProfile;


pub struct TankProfilesRepository;
impl TankProfilesRepository{
    pub fn select_tank_profiles(profile_tank_id: String) -> Vec<TankProfile>{
        let connection = &mut establish_connection();
        let tank_profiles = FilterDsl::filter(
            tank_profile, tank_id.eq(profile_tank_id)
        ).select(TankProfile::as_select())
            .load(connection).expect("Error during selecting tank profiles");
        tank_profiles
    }


    pub fn select_tank_profile(profile_tank_id: String, fuel_height: f64, tank_trim: f64) -> TankProfile{
        let connection = &mut establish_connection();
        let profile = FilterDsl::filter(
            tank_profile,
            tank_id.eq(profile_tank_id)
            .and(height.eq(fuel_height))
            .and(trim.eq(tank_trim))
        ).select(TankProfile::as_select())
            .get_result(connection).expect("Error during selecting tank profiles");
        profile
    }


    pub fn insert_tank_profile(new_tank_profile: TankProfile) -> TankProfile{
        let connection = &mut establish_connection();
        
        insert_into(tank_profile)
            .values(&new_tank_profile)
            .get_result(connection).expect("Error during insert tank profiles")
    }


    pub fn insert_tank_profiles(new_tank_profiles: Vec<TankProfile>) -> usize{
        let connection = &mut establish_connection();
        
        insert_into(tank_profile)
            .values(&new_tank_profiles)
            .execute(connection).expect("Error during insert tank")
    }
}