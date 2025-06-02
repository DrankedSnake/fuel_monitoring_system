diesel::table! {
    tank_profile (id) {
        id -> VarChar,
        tank_id -> VarChar,
        height -> Double,
        volume -> Double,
        trim -> Double,
    }
}
