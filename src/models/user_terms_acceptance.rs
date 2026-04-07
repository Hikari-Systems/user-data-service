use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserTermsAcceptance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub terms_version: String,
    pub accepted_at: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserTermsAcceptance {
    pub user_id: Uuid,
    pub terms_version: String,
}

pub async fn insert(
    pool: &PgPool,
    input: CreateUserTermsAcceptance,
) -> Result<UserTermsAcceptance> {
    let now = chrono::Utc::now().naive_utc();
    let row = sqlx::query_as::<_, UserTermsAcceptance>(
        "INSERT INTO user_terms_acceptance (id, user_id, terms_version, accepted_at, created_at)
         VALUES ($1, $2, $3, $4, $4)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(input.user_id)
    .bind(&input.terms_version)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<Option<UserTermsAcceptance>> {
    let row = sqlx::query_as::<_, UserTermsAcceptance>(
        "SELECT * FROM user_terms_acceptance WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<UserTermsAcceptance>> {
    let rows = sqlx::query_as::<_, UserTermsAcceptance>(
        "SELECT * FROM user_terms_acceptance WHERE user_id = $1 ORDER BY accepted_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_latest_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<UserTermsAcceptance>> {
    let row = sqlx::query_as::<_, UserTermsAcceptance>(
        "SELECT * FROM user_terms_acceptance WHERE user_id = $1 ORDER BY accepted_at DESC LIMIT 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM user_terms_acceptance WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
