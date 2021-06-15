#[macro_use]
extern crate diesel;

pub mod prelude {
    pub use crate::actix_stuff::handlers::routes;
    pub use crate::diesel_stuff::connection::{DbConnection, init, connect};
}

pub mod actix_stuff {
    pub mod handlers;
    pub mod models;   
}

pub mod diesel_stuff {
    pub mod connection;
    pub mod models;
    pub mod schema;
}