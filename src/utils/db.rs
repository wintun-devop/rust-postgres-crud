use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Db {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to Postgres")
}
