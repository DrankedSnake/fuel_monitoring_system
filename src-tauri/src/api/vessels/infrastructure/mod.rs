mod vessel;
mod vessel_schema;
mod repository;

pub use vessel::Vessel;
pub use vessel_schema::vessel as schema;
pub use repository::{
    select_vessels, insert_vessel
}; 