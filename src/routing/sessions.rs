use axum::{routing, Router, Extension};
use sqlx::PgPool;

use crate::{service::sessions::BcryptSessionService, control, repository::{sessions::PgSessionRepository, users::PgUserRepository}};

pub fn sessions_router(pool: &PgPool) -> Router {
    let session_service = BcryptSessionService::new(PgSessionRepository::new(pool), PgUserRepository::new(pool));

    let handler = routing::post(control::sessions::post_sessions::<BcryptSessionService<PgSessionRepository, PgUserRepository>>)
        .layer(Extension(session_service));

    Router::new()
        .route("/", handler)
}
