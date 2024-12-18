pub mod application;
pub mod domain;
mod infrastructure;

pub use application::{
    get_tank_profiles,
    add_tank_profile,
    add_tank_profiles,
    get_tank_profile,
    get_tank_profiles_amount,
};