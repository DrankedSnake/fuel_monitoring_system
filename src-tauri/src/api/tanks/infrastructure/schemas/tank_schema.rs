diesel::table! {
    tank (id) {
        id -> VarChar,
        name -> VarChar,
        available_volume -> Double,
        current_volume -> Double,
        vessel_id -> VarChar,
        previous_volume -> Double,
    }
}
