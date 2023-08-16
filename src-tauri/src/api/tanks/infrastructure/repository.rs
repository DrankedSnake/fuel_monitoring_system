use diesel::query_dsl::methods::FilterDsl;
use diesel::{QueryDsl, SelectableHelper, RunQueryDsl, insert_into, ExpressionMethods};

use super::super::super::fms_core::establish_connection;
use super::Tank;
use super::schema::dsl::*;


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




