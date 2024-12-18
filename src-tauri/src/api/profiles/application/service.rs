use core::panic;
use std::collections::HashMap;
use std::io;

use csv::Reader;
use log_derive::logfn;
use serde_json::Value;
use log::info;

use super::super::infrastructure::TankProfilesRepository;
use super::super::domain::{TankProfile, ProfileMeta};


pub struct TankProfileService;
impl TankProfileService{
    // #[logfn(Trace)]
    #[logfn(err = "Error", fmt = "Failed insert profiles: {:?}")]
    pub fn create_tank_profiles_from_csv_file(file_path: String, tank_id: &str) -> Result<(), io::Error>{
        let mut reader = Reader::from_path(file_path)?;
        info!("After reading the file");
        let headers = reader.headers().unwrap().clone();
        let records = reader.records();
        let empty_cell = String::from("");
        let mut profiles = Vec::new();
        info!("After preparing variables");
        for record in records {
            let record = record.unwrap();
            let mut index = 1;

            while index < record.len(){
                if profiles.len() == 10000 {
                    TankProfilesRepository::insert_tank_profiles(profiles);
                    profiles = vec![];
                }else{
                    match TankProfile::from_csv_record(
                        tank_id,
                        record[0].to_string(), 
                        headers[index].to_string(), 
                        record[index].to_string(),
                        &empty_cell,
                    ){
                        Some(profile) => profiles.push(profile),
                        None => ()
                    }
                    index += 1;
                }
            }
        }
        info!("After loop");
        if profiles.len() > 0 {
            TankProfilesRepository::insert_tank_profiles(profiles);
        }
        Ok(())
    }

    #[logfn(Trace)]
    pub fn add_tank_profile(data: HashMap<String, Value>) -> TankProfile {
        TankProfilesRepository::insert_tank_profile(
            TankProfile::from_map(data)
        )
    }

    pub fn get_tank_profiles(data: HashMap<String, Value>) -> Vec<TankProfile> {
        let profile_meta = ProfileMeta::from_map(data);
        println!("{:#?}", profile_meta);
        if matches!(profile_meta.height, Option::None) && matches!(profile_meta.trim, Option::None){
            return TankProfilesRepository::select_tank_profiles(profile_meta)
        } else if matches!(profile_meta.height, Option::Some(_f64)) && matches!(profile_meta.trim, Option::None) {
            return TankProfilesRepository::select_tank_profiles_by_height(profile_meta)
        } else if matches!(profile_meta.height, Option::None) && matches!(profile_meta.trim, Option::Some(_f64)) {
            return TankProfilesRepository::select_tank_profiles_by_trim(profile_meta)
        } else if matches!(profile_meta.height, Option::Some(_f64)) && matches!(profile_meta.trim, Option::Some(_f64)) {
            return TankProfilesRepository::select_tank_profiles_by_height_and_trim(profile_meta)
        } else {
            panic!("Exceptional case raised!")
        }

    }

    #[logfn(Trace)]
    pub fn get_tank_profile(tank_id: String, height: f64, trim: f64) -> Option<TankProfile> {
        TankProfilesRepository::select_tank_profile(
            tank_id, height, trim
        )
    }

    pub fn get_tank_profiles_amount(data: HashMap<String, Value>) -> i64 {
        let profile_meta = ProfileMeta::from_map(data);
        
        if matches!(profile_meta.height, Option::None) && matches!(profile_meta.trim, Option::None){
            return TankProfilesRepository::select_tank_profiles_count(profile_meta)
        } else if matches!(profile_meta.height, Option::Some(_f64)) && matches!(profile_meta.trim, Option::None) {
            return TankProfilesRepository::select_tank_profiles_by_height_count(profile_meta)
        } else if matches!(profile_meta.height, Option::None) && matches!(profile_meta.trim, Option::Some(_f64)) {
            return TankProfilesRepository::select_tank_profiles_by_trim_count(profile_meta)
        } else if matches!(profile_meta.height, Option::Some(_f64)) && matches!(profile_meta.trim, Option::Some(_f64)) {
            return TankProfilesRepository::select_tank_profiles_by_height_and_trim_count(profile_meta)
        } else {
            panic!("Exceptional case raised!")
        }
    }
}