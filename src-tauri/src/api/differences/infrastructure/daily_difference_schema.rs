diesel::table! {
    daily_difference (id) {
        id -> VarChar,
        vessel_id -> VarChar,
        volume -> Double,
        mass -> Double,
        date -> Date,
    }
}
