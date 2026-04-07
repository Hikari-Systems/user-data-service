use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{access_request, user};
use crate::AppState;

#[derive(Deserialize)]
pub(crate) struct ByEmailQuery {
    email: String,
}

#[derive(Deserialize)]
pub(crate) struct AccessRequestPath {
    id: Uuid,
    key: String,
}

#[derive(Deserialize)]
pub(crate) struct CreateAccessRequestPath {
    user_id: Uuid,
    key: String,
}

pub async fn get_by_email(
    state: web::Data<AppState>,
    query: web::Query<ByEmailQuery>,
) -> actix_web::Result<HttpResponse> {
    match user::get_by_email(&state.pool, &query.email)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(u) => Ok(HttpResponse::Ok().json(u)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    match user::get(&state.pool, path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(u) => Ok(HttpResponse::Ok().json(u)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<user::CreateUser>,
) -> actix_web::Result<HttpResponse> {
    let u = user::insert(&state.pool, body.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(u))
}

pub async fn upsert(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<user::UpsertUser>,
) -> actix_web::Result<HttpResponse> {
    let u = user::upsert(&state.pool, path.into_inner(), body.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(u))
}

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    match user::get(&state.pool, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        None => Ok(HttpResponse::NoContent().finish()),
        Some(_) => {
            user::delete(&state.pool, id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
            // 203 Non-Authoritative Information matches TS behaviour
            Ok(HttpResponse::NonAuthoritativeInformation().finish())
        }
    }
}

pub async fn get_access_requests(
    state: web::Data<AppState>,
    path: web::Path<AccessRequestPath>,
) -> actix_web::Result<HttpResponse> {
    let p = path.into_inner();
    let rows = access_request::get_by_user_id_and_key(&state.pool, p.id, &p.key)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn create_access_request(
    state: web::Data<AppState>,
    path: web::Path<CreateAccessRequestPath>,
) -> actix_web::Result<HttpResponse> {
    let p = path.into_inner();
    let ar = access_request::insert(&state.pool, p.user_id, &p.key)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(ar))
}

pub async fn delete_access_request(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    match access_request::get(&state.pool, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        None => Ok(HttpResponse::NoContent().finish()),
        Some(_) => {
            access_request::delete(&state.pool, id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok(HttpResponse::NonAuthoritativeInformation().finish())
        }
    }
}
