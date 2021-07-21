/// Diesel support
#[macro_use] extern crate diesel;

/// Used from the main function
pub use crate::connection::init;
pub use crate::handlers::routes;

mod handlers;
mod models;
mod connection;
mod schema;

mod useful {
    pub use crate::connection::connect;
    pub use crate::schema::*; // Re-export schema models
    pub use crate::diesel::*; // Re-export macros
    pub use crate::*;
}
