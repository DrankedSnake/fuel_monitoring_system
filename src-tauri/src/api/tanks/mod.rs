pub mod application;
pub mod domain;
mod infrastructure;

pub use application::{add_tank, get_tanks};
pub use infrastructure::Tank;
