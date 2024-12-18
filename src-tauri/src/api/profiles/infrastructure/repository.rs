use diesel::{prelude::*, insert_into};
use diesel::query_dsl::methods::FilterDsl;
use log_derive::{logfn, logfn_inputs};

use super::super::super::fms_core::establish_connection;
use super::schema;
use super::super::domain::{TankProfile, ProfileMeta};


pub struct TankProfilesRepository;
impl TankProfilesRepository{
    #[logfn(Trace)]
    pub fn select_tank_profiles(profile_meta: ProfileMeta) -> Vec<TankProfile>{
        let connection = &mut establish_connection();

        let tank_profiles = FilterDsl::filter(
            schema::table, schema::tank_id.eq(profile_meta.tank_id)
        )
        .limit(profile_meta.limit)
        .offset(profile_meta.offset)
        .load(connection).expect("Error during selecting tank profile");

        tank_profiles
    }

    #[logfn(Trace)]
    pub fn select_tank_profiles_by_height(profile_meta: ProfileMeta) -> Vec<TankProfile>{
        let connection = &mut establish_connection();

        let tank_profiles = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_meta.tank_id)
            .and(schema::height.eq(profile_meta.height.unwrap()))
        )
        .limit(profile_meta.limit)
        .offset(profile_meta.offset)
        .load(connection).expect("Error during selecting tank profile");

        tank_profiles
    }

    #[logfn(Trace)]
    pub fn select_tank_profiles_by_trim(profile_meta: ProfileMeta) -> Vec<TankProfile>{
        let connection = &mut establish_connection();

        let tank_profiles = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_meta.tank_id)
            .and(schema::trim.eq(profile_meta.trim.unwrap()))
        )
        .limit(profile_meta.limit)
        .offset(profile_meta.offset)
        .load(connection).expect("Error during selecting tank profile");

        tank_profiles
    }

    #[logfn(Trace)]
    pub fn select_tank_profiles_by_height_and_trim(profile_meta: ProfileMeta) -> Vec<TankProfile>{
        let connection = &mut establish_connection();

        let tank_profiles = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_meta.tank_id)
            .and(schema::height.eq(profile_meta.height.unwrap()))
            .and(schema::trim.eq(profile_meta.trim.unwrap()))
        )
        .limit(profile_meta.limit)
        .offset(profile_meta.offset)
        .load(connection).expect("Error during selecting tank profile");

        tank_profiles
    }

    #[logfn_inputs(Trace)]
    pub fn select_tank_profile(profile_tank_id: String, fuel_height: f64, tank_trim: f64) -> Option<TankProfile>{
        let connection = &mut establish_connection();
        let profile = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_tank_id)
            .and(schema::height.eq(fuel_height))
            .and(schema::trim.eq(tank_trim))
        )
        .select(TankProfile::as_select())
        .get_result(connection).optional().unwrap();
    
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

    #[logfn(Trace)]
    pub fn select_tank_profiles_count(profile_meta: ProfileMeta) -> i64 {
        let connection = &mut establish_connection();

        let result = FilterDsl::filter(
            schema::table, 
            schema::tank_id.eq(profile_meta.tank_id)
        ).count().get_result(connection).expect("Error during counting profiles");
        result
    }

    #[logfn(Trace)]
    pub fn select_tank_profiles_by_height_count(profile_meta: ProfileMeta) -> i64 {
        let connection = &mut establish_connection();

        let result = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_meta.tank_id)
            .and(schema::height.eq(profile_meta.height.unwrap()))
        ).count().get_result(connection).expect("Error during counting profiles");
        result
    }

    #[logfn(Trace)]
    pub fn select_tank_profiles_by_trim_count(profile_meta: ProfileMeta) -> i64 {
        let connection = &mut establish_connection();

        let result = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_meta.tank_id)
            .and(schema::trim.eq(profile_meta.trim.unwrap()))
        ).count().get_result(connection).expect("Error during counting profiles");
        result
    }

    #[logfn(Trace)]
    pub fn select_tank_profiles_by_height_and_trim_count(profile_meta: ProfileMeta) -> i64 {
        let connection = &mut establish_connection();

        let result = FilterDsl::filter(
            schema::table,
            schema::tank_id.eq(profile_meta.tank_id)
            .and(schema::height.eq(profile_meta.height.unwrap()))
            .and(schema::trim.eq(profile_meta.trim.unwrap()))
        ).count().get_result(connection).expect("Error during counting profiles");
        result
    }
}