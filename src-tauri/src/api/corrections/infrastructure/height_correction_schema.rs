diesel::table! {
    height_correction (id) {
        id -> VarChar,
        tank_id -> VarChar,
        trim -> Double,
        correction -> Double,
    }
}
