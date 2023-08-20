pub mod application;
pub mod domain;
mod infrastructure;

pub use application::{
    get_tanks, 
    add_tank,
};
