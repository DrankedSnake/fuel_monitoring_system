mod application;
mod domain;
mod infrastructure;

pub use application::{
    add_difference,
    get_differences,
    get_daily_differences_for_current_month,
    get_differences_amount,
};
