use diesel::pg::PgConnection;

use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection(database_url: String) -> PgPool {
    init_pool(&database_url).expect("Failed to create pool")
}