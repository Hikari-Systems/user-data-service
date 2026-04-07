use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::user_terms_acceptance::{self, CreateUserTermsAcceptance};
use crate::AppState;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    match user_terms_acceptance::get(&state.pool, path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(a) => Ok(HttpResponse::Ok().json(a)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

pub async fn get_by_user_id(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let rows = user_terms_acceptance::get_by_user_id(&state.pool, path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn get_latest_by_user_id(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    match user_terms_acceptance::get_latest_by_user_id(&state.pool, path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(a) => Ok(HttpResponse::Ok().json(a)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateUserTermsAcceptance>,
) -> actix_web::Result<HttpResponse> {
    let a = user_terms_acceptance::insert(&state.pool, body.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(a))
}

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    match user_terms_acceptance::get(&state.pool, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        None => Ok(HttpResponse::NoContent().finish()),
        Some(_) => {
            user_terms_acceptance::delete(&state.pool, id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok(HttpResponse::NonAuthoritativeInformation().finish())
        }
    }
}
