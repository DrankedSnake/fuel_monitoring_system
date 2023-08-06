diesel::table! {
    difference (id) {
        id -> VarChar,
        from_tank_id -> VarChar,
        to_tank_id -> VarChar,
        tanker_id -> VarChar,
        consumer_id -> VarChar,
        difference_type -> VarChar,
        fuel_volume -> Double,
        fuel_mass -> Double,
        density_coefficient_id -> VarChar,
        date_created -> Timestamp,
    }
}
