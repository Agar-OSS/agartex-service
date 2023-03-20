use axum::{routing, Router, Extension, http::HeaderValue};
use hyper::Method;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::{service::sessions::BcryptSessionService, control, constants, repository::{sessions::PgSessionRepository, users::PgUserRepository}};

pub fn sessions_router(pool: &PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(constants::CLIENT_URL.parse::<HeaderValue>().unwrap());

    let session_service = BcryptSessionService::new(PgSessionRepository::new(pool.clone()), PgUserRepository::new(pool.clone()));

    let handler = routing::post(control::sessions::post_sessions::<BcryptSessionService<PgSessionRepository, PgUserRepository>>)
        .layer(Extension(session_service));

    Router::new()
        .route("/", handler)
        .layer(cors)
}
