use csv::Reader;

use crate::api::tanks::{TankProfile, insert_tank_profiles};

pub fn create_tank_profiles_from_csv_file(
    file_path: String, tank_id: &str
){
    let mut reader = Reader::from_path(file_path).expect("No such file found.");
    let headers = reader.headers().unwrap().clone();
    let records = reader.records();
    let empty_cell = String::from("");
    let mut profiles = Vec::new();

    for record in records {
        let record = record.unwrap();
        let mut index = 1;

        while index < record.len(){
            if profiles.len() == 10000 {
                insert_tank_profiles(profiles);
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
    if profiles.len() > 0 {
        insert_tank_profiles(profiles);
    }
}