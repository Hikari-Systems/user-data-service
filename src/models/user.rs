use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub picture_image_service_id: Option<Uuid>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub email: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub picture: Option<String>,
    #[serde(default)]
    pub picture_image_service_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertUser {
    pub email: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub picture: Option<String>,
    #[serde(default)]
    pub picture_image_service_id: Option<Uuid>,
}

fn empty_string_as_none<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(d)?;
    Ok(opt.filter(|s| !s.is_empty()))
}

pub async fn insert(pool: &PgPool, input: CreateUser) -> Result<User> {
    let now = chrono::Utc::now().naive_utc();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO \"user\" (id, email, name, picture, picture_image_service_id, created_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&input.email)
    .bind(&input.name)
    .bind(&input.picture)
    .bind(input.picture_image_service_id)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn upsert(pool: &PgPool, id: Uuid, input: UpsertUser) -> Result<User> {
    let now = chrono::Utc::now().naive_utc();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO \"user\" (id, email, name, picture, picture_image_service_id, created_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (id) DO UPDATE SET
             email = EXCLUDED.email,
             name = EXCLUDED.name,
             picture = EXCLUDED.picture,
             picture_image_service_id = EXCLUDED.picture_image_service_id,
             updated_at = $7
         RETURNING *",
    )
    .bind(id)
    .bind(&input.email)
    .bind(&input.name)
    .bind(&input.picture)
    .bind(input.picture_image_service_id)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM \"user\" WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM \"user\" WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM \"user\" WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
