diesel::table! {
    vessel (id) {
        id -> VarChar,
        name -> VarChar,
        year -> Integer,
        dead_weight -> Integer,
        imo -> VarChar,
        company -> VarChar,
    }
}
