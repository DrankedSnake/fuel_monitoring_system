use diesel::query_dsl::methods::FilterDsl;
use diesel::{QueryDsl, insert_into, ExpressionMethods, SelectableHelper, RunQueryDsl};
use log_derive::{logfn, logfn_inputs};

use super::super::super::fms_core::establish_connection;
use super::super::domain::TankCorrection;
use super::schema::dsl;


pub struct TankCorrectionsRepository;

impl TankCorrectionsRepository{
    #[logfn_inputs(INFO, fmt = "Searching corrections for tank {}")]
    pub fn select_tank_corrections(tank_id: String) -> Vec<TankCorrection>{
        let connection = &mut establish_connection();
        let tanks = FilterDsl::filter(dsl::height_correction, dsl::tank_id.eq(tank_id))
            .select(TankCorrection::as_select())
            .load(connection).expect("Error during selecting tank corrections");
        tanks
    }
}