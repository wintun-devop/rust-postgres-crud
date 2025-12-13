use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::PgPool;
use sqlx::Error as SqlxError;

type Db = PgPool;

#[derive(Debug,Serialize, Deserialize, sqlx::FromRow)]
pub struct Items {
    pub id: String,
    pub name: String,
    pub model: String,
}

#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub model: String,
}

pub async fn create_item(payload: CreateItem, db: &Db) -> Result<Items, SqlxError> {
    // generate id (or omit and let DB generate it)
    let new_id = Uuid::new_v4();

    // Use query_as with explicit RETURNING columns
    let item = sqlx::query_as::<_, Items>(
        r#"
        INSERT INTO esm_items (id, name, model)
        VALUES ($1, $2, $3)
        RETURNING id, name, model
        "#
    )
    .bind(new_id)
    .bind(&payload.name)
    .bind(&payload.model)
    .fetch_one(db)
    .await?;
    Ok(item)
}
