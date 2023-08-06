mod models;
mod vessel_repository;

pub use models::Vessel;
pub use vessel_repository::{select_vessels, insert_vessel}; 