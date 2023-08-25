use diesel::pg::PgConnection;
use diesel::prelude::*;


pub fn establish_connection() -> PgConnection {
    PgConnection::establish("postgres://postgres:postgres@localhost:5432/fms").unwrap()
}