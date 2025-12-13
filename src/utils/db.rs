use sqlx::PgPool;
use tokio::sync::OnceCell;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_db(database_url: &str) -> Result<(), sqlx::Error> {
    // If already initialized, do nothing
    if POOL.get().is_some() {
        return Ok(());
    }
    // Create the pool (propagate sqlx errors)
    let pool = PgPool::connect(database_url).await?;

    // Try to set the global OnceCell. If another task set it concurrently, ignore.
    let _ = POOL.set(pool);
    Ok(())
}



pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    if POOL.get().is_none() {
        // lazy initialize using config if not already initialized
        let database_url = crate::config::config().db_url;
        init_db(&database_url).await?;
    }
    Ok(POOL.get().expect("pool initialized").clone())
}