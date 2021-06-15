use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager; // Para poder utilizar r2d2 se especifica el feature "r2d2" en el Cargo.toml

use lazy_static::lazy_static;

// r2d2, encargado de manejar el "pozo" de conexiones a la base de datos
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub trait Poolable {
    fn init() -> Self;
}

lazy_static! {
    static ref POOL: Pool = {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn connect() -> DbConnection {
    POOL.get().unwrap()
}

pub fn init() {
    lazy_static::initialize(&POOL);
}