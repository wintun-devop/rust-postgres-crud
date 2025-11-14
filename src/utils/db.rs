use sqlx::{Pool, Postgres};
pub type Db = Pool<Postgres>;
use crate::config::config;

pub async fn init_db() -> Db {
    let url = config().db_url;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to Postgres");
    println!("✅ Connected to Postgres at {}", url);
    pool
}
