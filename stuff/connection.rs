use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;

// The alias for both the Pool and a single connection retrieved from the pool
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::new(database_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn connect() -> DbConnection { POOL.get().unwrap() }
pub fn init() { lazy_static::initialize(&POOL) }
