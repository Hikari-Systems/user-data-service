use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OauthProfile {
    pub sub: String,
    pub user_id: Uuid,
    #[sqlx(json)]
    pub profile_json: Value,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// profileJson arrives as a JSON-encoded string in the request body (matching TS behaviour).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertOauthProfile {
    pub sub: String,
    pub user_id: Uuid,
    pub profile_json: String,
}

pub async fn upsert(pool: &PgPool, input: UpsertOauthProfile) -> Result<OauthProfile> {
    let now = chrono::Utc::now().naive_utc();
    let parsed: Value = serde_json::from_str(&input.profile_json)?;
    let profile = sqlx::query_as::<_, OauthProfile>(
        "INSERT INTO oauth_profile (sub, user_id, profile_json, created_at)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (sub) DO UPDATE SET
             user_id = EXCLUDED.user_id,
             profile_json = EXCLUDED.profile_json,
             updated_at = $5
         RETURNING *",
    )
    .bind(&input.sub)
    .bind(input.user_id)
    .bind(sqlx::types::Json(&parsed))
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(profile)
}

pub async fn get_by_sub(pool: &PgPool, sub: &str) -> Result<Option<OauthProfile>> {
    let profile =
        sqlx::query_as::<_, OauthProfile>("SELECT * FROM oauth_profile WHERE sub = $1")
            .bind(sub)
            .fetch_optional(pool)
            .await?;
    Ok(profile)
}

pub async fn get_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<OauthProfile>> {
    let profile =
        sqlx::query_as::<_, OauthProfile>("SELECT * FROM oauth_profile WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
    Ok(profile)
}

pub async fn delete(pool: &PgPool, sub: &str) -> Result<()> {
    sqlx::query("DELETE FROM oauth_profile WHERE sub = $1")
        .bind(sub)
        .execute(pool)
        .await?;
    Ok(())
}
