diesel::table! {
    vessel (id) {
        id -> VarChar,
        name -> VarChar,
        year -> Integer,
    }
}