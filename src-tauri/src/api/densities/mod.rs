pub mod application;
pub mod domain;
mod infrastructure;

pub use application::{
    add_density_coefficient, add_density_coefficients, get_density_coefficient_in_air,
    get_density_coefficient_in_vacuum, get_density_coefficients, get_density_coefficients_amount,
};
