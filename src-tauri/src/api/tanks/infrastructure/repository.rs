use diesel::query_dsl::methods::FilterDsl;
use diesel::{QueryDsl, insert_into, ExpressionMethods, SelectableHelper, RunQueryDsl, update};

use super::super::super::fms_core::establish_connection;
use super::super::domain::Tank;
use super::schema::dsl;

pub struct TanksRepository;
impl TanksRepository{
    pub fn select_tanks(tank_vessel_id: String) -> Vec<Tank>{
        let connection = &mut establish_connection();
        let tanks = FilterDsl::filter(dsl::tank, dsl::vessel_id.eq(tank_vessel_id))
            .select(Tank::as_select())
            .load(connection).expect("Error during selecting tanks");
        tanks
    }

    pub fn insert_tank(tank: Tank) -> Tank{
        let connection = &mut establish_connection();
        
        let result = insert_into(dsl::tank)
            .values(&tank)
            .get_result(connection).expect("Error during insert tank");
        result
    }

    pub fn select_tank(id: String) -> Tank {
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(dsl::tank, dsl::id.eq(id))
            .select(Tank::as_select())
            .get_result(connection).expect("Error during selecting tanks");
        result
    }

    pub fn update_tank(updated_tank: Tank) -> Tank {
        let connection = &mut establish_connection();
        
        update(dsl::tank).filter(dsl::id.eq(updated_tank.id)).set(
            (
                dsl::current_mass.eq(updated_tank.current_mass),
                dsl::previous_volume.eq(updated_tank.previous_volume),
                dsl::current_volume.eq(updated_tank.current_volume),
                dsl::previous_mass.eq(updated_tank.previous_mass),
            )
        ).get_result(connection).expect("Error during update tank")
    }
}