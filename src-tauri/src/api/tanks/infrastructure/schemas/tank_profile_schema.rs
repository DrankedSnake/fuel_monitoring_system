diesel::table! {
    tank_profile (id) {
        id -> VarChar,
        tank_id -> VarChar,
        height -> Double,
        trim -> Double,
        volume -> Double,
    }
}