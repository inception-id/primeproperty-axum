use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn build_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
