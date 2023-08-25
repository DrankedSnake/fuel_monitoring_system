use diesel::{prelude::*, insert_into};
use diesel::query_dsl::methods::FilterDsl;
use log_derive::{logfn, logfn_inputs};

use super::super::super::fms_core::establish_connection;
use super::schema;
use super::super::domain::TankProfile;

pub struct TankProfilesRepository;
impl TankProfilesRepository{
    #[logfn_inputs(INFO, fmt = "Searching profiles for tank {}")]
    pub fn select_tank_profiles(profile_tank_id: String) -> Vec<TankProfile>{
        let connection = &mut establish_connection();

        let tank_profiles = FilterDsl::filter(
            schema::table, schema::tank_id.eq(profile_tank_id)
        )
        .limit(15)
        .offset(0)
        .load(connection).expect("Error during selecting tank profile");
        tank_profiles
    }
 
    #[logfn(Trace)]
    pub fn select_tank_profile(profile_tank_id: String, fuel_height: f64, tank_trim: f64) -> TankProfile{
        let connection = &mut establish_connection();
        let profile = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_tank_id)
            .and(schema::height.eq(fuel_height))
            .and(schema::trim.eq(tank_trim))
        )
        .select(TankProfile::as_select())
        .get_result(connection).expect("Error during selecting tank profiles");
        profile
    }

    #[logfn(Trace)]
    pub fn insert_tank_profile(new_tank_profile: TankProfile) -> TankProfile{
        let connection = &mut establish_connection();
        
        insert_into(schema::table)
        .values(&new_tank_profile)
        .get_result(connection).expect("Error during insert tank profiles")
    }

    #[logfn(Trace)]
    pub fn insert_tank_profiles(new_tank_profiles: Vec<TankProfile>) -> usize{
        let connection = &mut establish_connection();
        
        insert_into(schema::table)
        .values(&new_tank_profiles)
        .execute(connection).expect("Error during insert tank")
    }
}