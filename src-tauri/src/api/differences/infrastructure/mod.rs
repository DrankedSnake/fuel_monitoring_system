mod difference;
mod difference_schema;
mod repository;

pub use difference::Difference;
pub use difference_schema::difference as schema;
pub use repository::{
    insert_difference,
    select_differences
};