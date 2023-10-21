diesel::table! {
    density_coefficient (id) {
        id -> VarChar,
        temperature -> Double,
        density -> Double,
        coefficient -> Double,
        factor -> VarChar,
    }
}
