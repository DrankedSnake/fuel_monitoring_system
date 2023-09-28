diesel::table! {
    daily_difference (id) {
        id -> VarChar,
        vessel_id -> VarChar,
        hfo_volume -> Double,
        hfo_mass -> Double,
        mgo_volume -> Double,
        mgo_mass -> Double,
        date -> Date,
    }
}
