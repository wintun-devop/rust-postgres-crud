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

/// Get a clone of the global pool. Panics if init_db was not called.
pub fn get_pool() -> PgPool {
    POOL
        .get()
        .expect("POOL not initialized; call init_db() in main before using get_pool()")
        .clone()
}
