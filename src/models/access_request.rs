use anyhow::Result;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AccessRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key: String,
    pub granted: Option<bool>,
    pub decided_at: Option<NaiveDateTime>,
    pub granted_from: Option<NaiveDateTime>,
    pub granted_until: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub async fn insert(pool: &PgPool, user_id: Uuid, key: &str) -> Result<AccessRequest> {
    let now = chrono::Utc::now().naive_utc();
    let ar = sqlx::query_as::<_, AccessRequest>(
        "INSERT INTO access_request
             (id, user_id, key, granted, decided_at, granted_from, granted_until, created_at)
         VALUES ($1, $2, $3, NULL, NULL, NULL, NULL, $4)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(key)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(ar)
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<Option<AccessRequest>> {
    let row = sqlx::query_as::<_, AccessRequest>(
        "SELECT * FROM access_request WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM access_request WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_by_user_id_and_key(
    pool: &PgPool,
    user_id: Uuid,
    key: &str,
) -> Result<Vec<AccessRequest>> {
    let rows = sqlx::query_as::<_, AccessRequest>(
        "SELECT * FROM access_request WHERE user_id = $1 AND key = $2 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .bind(key)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
