use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::env;

pub type DbPool = Pool<Sqlite>;

pub async fn init_db() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
