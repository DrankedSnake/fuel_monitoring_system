mod tank_schema;
mod repository;
mod model;

pub use repository::TanksRepository;
pub use tank_schema::tank as schema;
pub use model::Tank;