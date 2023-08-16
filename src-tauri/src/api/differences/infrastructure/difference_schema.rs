diesel::table! {
    difference (id) {
        id -> VarChar,
        tank_id -> VarChar,
        volume -> Double,
        mass -> Double,
        density_coefficient_id -> VarChar,
        date_created -> Timestamptz,
    }
}
