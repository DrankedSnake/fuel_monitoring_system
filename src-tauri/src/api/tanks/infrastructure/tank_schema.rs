diesel::table! {
    tank (id) {
        id -> VarChar,
        name -> VarChar,
        full_volume -> Double,
        current_volume -> Double,
        safe_volume -> Double,
        vessel_id -> VarChar,
        previous_volume -> Double,
        current_mass -> Double,
        previous_mass -> Double,
        fuel_type -> VarChar,
        tank_type -> VarChar,
    }
}