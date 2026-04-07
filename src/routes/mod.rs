use actix_web::web;

mod oauth_profile;
mod user;
mod user_terms_acceptance;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // --- user ---
        // Static/specific routes before wildcard /{id}
        .route("/api/user/byEmail", web::get().to(user::get_by_email))
        .route(
            "/api/user/{id}/accessRequest/{key}",
            web::get().to(user::get_access_requests),
        )
        .route(
            "/api/user/{user_id}/accessRequest/{key}",
            web::post().to(user::create_access_request),
        )
        .route("/api/user/{id}", web::get().to(user::get))
        .route("/api/user/{id}", web::put().to(user::upsert))
        .route("/api/user/{id}", web::delete().to(user::delete))
        .route("/api/user", web::post().to(user::create))
        .route(
            "/api/accessRequest/{id}",
            web::delete().to(user::delete_access_request),
        )
        // --- oauthProfile ---
        .route(
            "/api/oauthProfile/byUserId/{user_id}",
            web::get().to(oauth_profile::get_by_user_id),
        )
        .route(
            "/api/oauthProfile/bySub",
            web::get().to(oauth_profile::get_by_sub),
        )
        .route("/api/oauthProfile", web::put().to(oauth_profile::upsert))
        .route("/api/oauthProfile", web::delete().to(oauth_profile::delete))
        // --- userTermsAcceptance ---
        // /byUserId/{userId}/latest and /byUserId/{userId} must come before /{id}
        .route(
            "/api/userTermsAcceptance/byUserId/{user_id}/latest",
            web::get().to(user_terms_acceptance::get_latest_by_user_id),
        )
        .route(
            "/api/userTermsAcceptance/byUserId/{user_id}",
            web::get().to(user_terms_acceptance::get_by_user_id),
        )
        .route(
            "/api/userTermsAcceptance/{id}",
            web::get().to(user_terms_acceptance::get),
        )
        .route(
            "/api/userTermsAcceptance/{id}",
            web::delete().to(user_terms_acceptance::delete),
        )
        .route(
            "/api/userTermsAcceptance",
            web::post().to(user_terms_acceptance::create),
        );
}
