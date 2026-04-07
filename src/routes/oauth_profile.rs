use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::oauth_profile;
use crate::AppState;

#[derive(Deserialize)]
pub(crate) struct ByUserIdPath {
    user_id: Uuid,
}

#[derive(Deserialize)]
pub(crate) struct BySubQuery {
    sub: String,
}

pub async fn get_by_user_id(
    state: web::Data<AppState>,
    path: web::Path<ByUserIdPath>,
) -> actix_web::Result<HttpResponse> {
    match oauth_profile::get_by_user_id(&state.pool, path.into_inner().user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(p) => Ok(HttpResponse::Ok().json(p)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

pub async fn get_by_sub(
    state: web::Data<AppState>,
    query: web::Query<BySubQuery>,
) -> actix_web::Result<HttpResponse> {
    match oauth_profile::get_by_sub(&state.pool, &query.sub)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(p) => Ok(HttpResponse::Ok().json(p)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

pub async fn upsert(
    state: web::Data<AppState>,
    body: web::Json<oauth_profile::UpsertOauthProfile>,
) -> actix_web::Result<HttpResponse> {
    let p = oauth_profile::upsert(&state.pool, body.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(p))
}

pub async fn delete(
    state: web::Data<AppState>,
    query: web::Query<BySubQuery>,
) -> actix_web::Result<HttpResponse> {
    match oauth_profile::get_by_sub(&state.pool, &query.sub)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        None => Ok(HttpResponse::NoContent().finish()),
        Some(_) => {
            oauth_profile::delete(&state.pool, &query.sub)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok(HttpResponse::NonAuthoritativeInformation().finish())
        }
    }
}
