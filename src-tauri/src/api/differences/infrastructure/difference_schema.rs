diesel::table! {
    difference (id) {
        id -> VarChar,
        tank_id -> VarChar,
        volume -> Double,
        mass -> Double,
        date_created -> Timestamptz,
        height -> Double,
        trim -> Double,
        temperature -> Double,
        density -> Double,
        fuel_type -> VarChar,
    }
}
