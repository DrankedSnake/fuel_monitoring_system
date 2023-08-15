diesel::table! {
    difference (id) {
        id -> VarChar,
        tank_id -> VarChar,
        difference_type -> VarChar,
        volume -> Float,
        mass -> Float,
        density_coefficient_id -> VarChar,
        date_created -> Datetime,
    }
}
