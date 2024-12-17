use diesel::prelude::*;

pub fn establish_connection() -> deadpool_diesel::postgres::Pool{
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    match deadpool_diesel::postgres::Pool::builder(manager).build() {
       Ok(pool) => pool,
        Err(e) => panic!("deadpool builtin pool error: {:?}", e),
    }
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // PgConnection::establish(&database_url)
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}