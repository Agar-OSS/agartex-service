use axum::{routing, Router, Extension};
use sqlx::PgPool;

use crate::{service::{sessions::HashSessionService, hash::BcryptHashService}, control::sessions, repository::{sessions::PgSessionRepository, users::PgUserRepository}};

pub fn sessions_router(pool: &PgPool) -> Router {
    let session_service = HashSessionService::new(
        PgSessionRepository::new(pool), 
        PgUserRepository::new(pool), 
        BcryptHashService::new()
    );

    let handler = routing::post(sessions::post_sessions::<HashSessionService<PgSessionRepository, PgUserRepository, BcryptHashService>>)
        .layer(Extension(session_service));

    Router::new()
        .route("/", handler)
}
