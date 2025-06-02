use diesel::query_dsl::methods::FilterDsl;
use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use log_derive::logfn;

use super::super::super::fms_core::establish_connection;
use super::schema;
use super::Tank;

pub struct TanksRepository;
impl TanksRepository {
    #[logfn(Trace)]
    pub fn select_tanks(tank_vessel_id: String) -> Vec<Tank> {
        let connection = &mut establish_connection();
        let tanks = FilterDsl::filter(schema::table, schema::vessel_id.eq(tank_vessel_id))
            .select(Tank::as_select())
            .load(connection)
            .expect("Error during selecting tanks");
        tanks
    }

    #[logfn(Trace)]
    pub fn insert_tank(tank: Tank) -> Tank {
        let connection = &mut establish_connection();

        let result = insert_into(schema::table)
            .values(&tank)
            .get_result(connection)
            .expect("Error during insert tank");
        result
    }

    #[logfn(Trace)]
    pub fn select_tank(id: String) -> Tank {
        let connection = &mut establish_connection();
        let result = FilterDsl::filter(schema::table, schema::id.eq(id))
            .select(Tank::as_select())
            .get_result(connection)
            .expect("Error during selecting tanks");
        result
    }

    #[logfn(Trace)]
    pub fn update_tank(updated_tank: Tank) -> Tank {
        let connection = &mut establish_connection();

        update(schema::table)
            .filter(schema::id.eq(updated_tank.id))
            .set((
                schema::current_mass.eq(updated_tank.current_mass),
                schema::previous_volume.eq(updated_tank.previous_volume),
                schema::current_volume.eq(updated_tank.current_volume),
                schema::previous_mass.eq(updated_tank.previous_mass),
            ))
            .get_result(connection)
            .expect("Error during update tank")
    }
}
