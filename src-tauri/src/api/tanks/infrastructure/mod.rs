mod tank;
mod tank_schema;
mod repository;

pub use repository::{
    select_tanks,
    insert_tank,
};
pub use tank::Tank;
pub use tank_schema::tank as schema;