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

#[derive(Deserialize)]
pub struct UpdateItem {
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

/// Get all items
pub async fn get_all_items(db: &Db) -> Result<Vec<Items>, SqlxError> {
    let items = sqlx::query_as::<_, Items>(
        r#"
        SELECT id, name, model
        FROM esm_items
        ORDER BY name
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(items)
}

// get one by id
pub async fn get_item_by_id(id: &str, db: &Db) -> Result<Option<Items>, SqlxError> {
    let item = sqlx::query_as::<_, Items>(
        r#"
        SELECT id, name, model
        FROM esm_items
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(item)
}

/// Update item (partial update). Returns the updated row.
pub async fn update_item(id: &str, payload: UpdateItem, db: &Db) -> Result<Items, SqlxError> {
    // COALESCE will keep existing value when payload field is NULL
    let item = sqlx::query_as::<_, Items>(
        r#"
        UPDATE esm_items
        SET
            name  = COALESCE($2, name),
            model = COALESCE($3, model)
        WHERE id = $1
        RETURNING id, name, model
        "#
    )
    .bind(id)
    .bind(payload.name)   // Option<String> binds as NULL or value
    .bind(payload.model)  // Option<String>
    .fetch_one(db)
    .await?;
    Ok(item)
}

/// Delete item by id. Returns true if a row was deleted.
pub async fn delete_item(id: &str, db: &Db) -> Result<bool, SqlxError> {
    let result = sqlx::query(
        r#"
        DELETE FROM esm_items
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(db)
    .await?;
    Ok(result.rows_affected() > 0)
}